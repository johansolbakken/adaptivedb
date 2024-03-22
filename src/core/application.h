#pragma once

#include "core/base.h"
#include "core/server.h"

#include "catalogue/catalogue.h"

namespace AdaptiveDB {
class Application {
public:
  Application();
  ~Application();

  void run();

  void stop() { m_running = false; }

  static Application& instance() { return *m_instance; }

  Ref<Server> server() { return m_server; }
  Ref<Catalogue> catalogue() { return m_catalogue; }

private:
  Ref<Server> m_server;
  Ref<Catalogue> m_catalogue;

  bool m_running = false;
  static Application* m_instance;
};
} // namespace AdaptiveDB