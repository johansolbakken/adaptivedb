#include "application.h"

#include "handler/index.h"

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

        m_server->get("/", index);

        m_server->run(3000);

        m_running = true;
        while (m_running) {
            m_server->update();
        }
    }
} // namespace AdaptiveDB