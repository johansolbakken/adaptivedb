#include "server.h"

#include "core/base.h"

#include <sys/socket.h>
#include <netinet/in.h>
#include <unistd.h>
#include <poll.h>

namespace AdaptiveDB
{
    Server::Server()
    {
    }

    Server::~Server()
    {
        m_running = false;
        close(m_serverSocket);
        Log::info("Server stopped");
    }

    // TODO: If path exists print warning

    void Server::get(const std::string &path, std::function<void(Request &req, nlohmann::json &res)> callback)
    {
        routes[Method::GET][path] = callback;
    }

    void Server::post(const std::string &path, std::function<void(Request &req, nlohmann::json &res)> callback)
    {
        routes[Method::POST][path] = callback;
    }

    void Server::put(const std::string &path, std::function<void(Request &req, nlohmann::json &res)> callback)
    {
        routes[Method::PUT][path] = callback;
    }

    void Server::del(const std::string &path, std::function<void(Request &req, nlohmann::json &res)> callback)
    {
        routes[Method::DELETE][path] = callback;
    }

    void Server::run(int port)
    {
        m_port = port;

        m_serverSocket = socket(AF_INET, SOCK_STREAM, 0);
        if (m_serverSocket == -1)
        {
            Log::error("Can't create socket");
            return;
        }

        sockaddr_in serverAddress;
        serverAddress.sin_family = AF_INET;
        serverAddress.sin_port = htons(m_port);
        serverAddress.sin_addr.s_addr = INADDR_ANY;

        if (bind(m_serverSocket, (sockaddr *)&serverAddress, sizeof(serverAddress)) == -1)
        {
            Log::error("Can't bind socket");
            return;
        }

        if (listen(m_serverSocket, 10) == -1)
        {
            Log::error("Can't listen on socket");
            return;
        }

        Log::info(fmt::format("Server is running on port {}", m_port));

        m_running = true;
    }

    void Server::update()
    {
        bool hasClient = false;
        pollfd pollFd;
        pollFd.fd = m_serverSocket;
        pollFd.events = POLLIN;
        pollFd.revents = 0;

        if (poll(&pollFd, 1, 1000) > 0)
        {
            hasClient = true;
        }

        if (!hasClient)
        {
            return;
        }

        int clientSocket = accept(m_serverSocket, nullptr, nullptr);
        if (clientSocket == -1)
        {
            Log::error("Can't accept client socket");
            return;
        }

        char buffer[4096];
        int bytesRead = recv(clientSocket, buffer, 4096, 0);

        if (bytesRead == -1)
        {
            Log::error("Can't read from client socket");
            return;
        }

        Request req;
        req.method = Method::GET;
        req.path = "/";
        req.body = nlohmann::json::object();

        std::string request = std::string(buffer, bytesRead);
        std::string requestMethod = request.substr(0, request.find(" "));
        std::string requestPath = request.substr(request.find(" ") + 1, request.find(" ", request.find(" ") + 1) - request.find(" ") - 1);

        size_t bodyStart = request.find("\r\n\r\n");
        if (bodyStart != std::string::npos)
        {
            std::string body = request.substr(bodyStart + 2);
            req.body = nlohmann::json::parse(body);
        }

        if (requestMethod == "GET")
        {
            req.method = Method::GET;
        }
        else if (requestMethod == "POST")
        {
            req.method = Method::POST;
        }
        else if (requestMethod == "PUT")
        {
            req.method = Method::PUT;
        }
        else if (requestMethod == "DELETE")
        {
            req.method = Method::DELETE;
        }

        req.path = requestPath;

        std::string response = "HTTP/1.1 200 OK\nContent-Type: application/json\n\n";
        nlohmann::json res = nlohmann::json::object();

        if (routes[req.method].find(req.path) != routes[req.method].end())
        {
            routes[req.method][req.path](req, res);
        }
        else
        {
            response = "HTTP/1.1 404 Not Found\nContent-Type: application/json\n\n";
            res["error"] = "Not found";
        }

        response += res.dump() + "\n";

        send(clientSocket, response.c_str(), response.size(), 0);

        close(clientSocket);
    }
}