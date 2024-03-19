#pragma once

#include <map>
#include <functional>
#include <queue>
#include <thread>

#include "nlohmann/json.hpp"

enum class Method
{
  GET,
  POST,
  PUT,
  DELETE
};

struct Request
{
  Method method;
  std::string path;
  nlohmann::json body;
};

namespace AdaptiveDB
{
  class Server
  {
  public:
    Server();
    ~Server();

    void get(const std::string &path, std::function<void(Request &req, nlohmann::json &res)> callback);
    void post(const std::string &path, std::function<void(Request &req, nlohmann::json &res)> callback);
    void put(const std::string &path, std::function<void(Request &req, nlohmann::json &res)> callback);
    void del(const std::string &path, std::function<void(Request &req, nlohmann::json &res)> callback);

    void run(int port = 3000);
    void update();

  private:
    std::map<Method, std::map<std::string, std::function<void(Request &req, nlohmann::json &res)>>> routes;
    int m_port = 3000;
    int m_serverSocket = 0;
    bool m_running = false;
  };
} // namespace AdaptiveDB