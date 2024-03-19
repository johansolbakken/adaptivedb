#include "application.h"

#include <iostream>

namespace AdaptiveDB {
Application::Application() {}
Application::~Application() {}
void Application::run() { std::cout << "Hello, AdaptiveDB!" << std::endl; }
} // namespace AdaptiveDB