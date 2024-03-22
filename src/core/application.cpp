#include "application.h"

#include "handler/index.h"
#include "handler/schema.h"
#include "handler/catalogue.h"

#include <sys/signal.h>

namespace AdaptiveDB
{
    Application *Application::m_instance = nullptr;

    Application::Application()
        : m_server(createRef<Server>()),
          m_catalogue(createRef<Catalogue>())
    {
        if (!m_instance)
        {
            m_instance = this;
        }
        else
        {
            Log::error("Error: Application already exists. Exiting...");
            std::exit(1);
        }

        // Control-C handler. We want to for instance close the server when the user presses Control-C
        signal(SIGINT, [](int)
               { Application::instance().stop(); });
    }

    Application::~Application()
    {
    }

    void Application::run()
    {
        auto version = versionConfig();
        Log::info(fmt::format("AdaptiveDB v{}.{}.{}", version.major, version.minor, version.patch));

        m_server->get("/", index);
        m_server->post("/schema", schema);
        m_server->get("/catalogue", getAllTables);
        m_server->post("/catalogue", createTableBySchema);

        m_server->run(3000);

        m_running = true;
        while (m_running)
        {
            m_server->update();
        }
    }
} // namespace AdaptiveDB