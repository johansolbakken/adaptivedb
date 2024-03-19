#include "server.h"

#include <sys/socket.h>
#include <netinet/in.h>
#include <unistd.h>

#include <iostream>

namespace AdaptiveDB
{
    Server::Server()
    {
    }

    Server::~Server()
    {
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
            std::cerr << "Error: Can't create socket" << std::endl;
            return;
        }

        sockaddr_in serverAddress;
        serverAddress.sin_family = AF_INET;
        serverAddress.sin_port = htons(m_port);
        serverAddress.sin_addr.s_addr = INADDR_ANY;

        if (bind(m_serverSocket, (sockaddr *)&serverAddress, sizeof(serverAddress)) == -1)
        {
            std::cerr << "Error: Can't bind socket" << std::endl;
            return;
        }

        if (listen(m_serverSocket, 10) == -1)
        {
            std::cerr << "Error: Can't listen socket" << std::endl;
            return;
        }

        std::cout << "Server is running on port " << m_port << std::endl;

        while (true)
        {
            sockaddr_in clientAddress;
            socklen_t clientAddressSize = sizeof(clientAddress);
            int clientSocket = accept(m_serverSocket, (sockaddr *)&clientAddress, &clientAddressSize);

            if (clientSocket == -1)
            {
                std::cerr << "Error: Can't accept client" << std::endl;
                continue;
            }

            char buffer[4096];
            int bytesRead = recv(clientSocket, buffer, 4096, 0);

            if (bytesRead == -1)
            {
                std::cerr << "Error: Can't read from socket" << std::endl;
                continue;
            }

            Request req;
            req.method = Method::GET;
            req.path = "/";
            req.body = nlohmann::json::object();

            std::string request = std::string(buffer, bytesRead);
            std::string requestMethod = request.substr(0, request.find(" "));
            std::string requestPath = request.substr(request.find(" ") + 1, request.find(" ", request.find(" ") + 1) - request.find(" ") - 1);

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

            response += res.dump();

            send(clientSocket, response.c_str(), response.size() + 1, 0);

            close(clientSocket);
        }
    }
}