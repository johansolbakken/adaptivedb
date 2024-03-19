#pragma once

#include "core/base.h"
#include "core/server.h"

namespace AdaptiveDB {
class Application {
public:
  Application();
  ~Application();

  void run();

private:
  Ref<Server> m_server;
  bool m_running = false;
};
} // namespace AdaptiveDB