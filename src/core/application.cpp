#include "application.h"

#include <iostream>

namespace AdaptiveDB
{
    Application::Application()
        : m_server(createRef<Server>())
    {
    }

    Application::~Application() {}

    void Application::run()
    {
        auto version = versionConfig();
        std::cout << "AdaptiveDB v" << version.major << "." << version.minor << "." << version.patch << std::endl;
        
        m_server->get("/", [](Request &req, nlohmann::json &res) {
            res["message"] = "Hello, World!";
        });

        m_server->run();
    }
} // namespace AdaptiveDB