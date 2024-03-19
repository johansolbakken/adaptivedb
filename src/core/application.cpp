#include "application.h"

#include "handler/index.h"

#include <iostream>
#include <sys/signal.h>

namespace AdaptiveDB
{
    Application *Application::m_instance = nullptr;

    Application::Application()
        : m_server(createRef<Server>())
    {
        if (!m_instance) {
            m_instance = this;
        } else {
            std::cerr << "Error: Application already exists" << std::endl;
            std::exit(1);
        }

        signal(SIGINT, [](int) {
            Application::instance().stop();
        });
    }

    Application::~Application() {

    }

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