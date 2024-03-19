#include <string>
#include <fmt/core.h>
#include <fmt/color.h>

#include "config/logcolor.h"

namespace AdaptiveDB
{
    class Log
    {
    public:
        static void info(const std::string &message)
        {
            fmt::print(LogColor::info, "[INFO] ");
            fmt::print("{}\n", message); // This line
        }

        static void warn(const std::string &message)
        {
            fmt::print(LogColor::warn, "[WARN] ");
            fmt::print("{}\n", message); // And this line
        }

        static void error(const std::string &message)
        {
            fmt::print(LogColor::error, "[ERROR] ");
            fmt::print("{}\n", message); // And here
        }
    };
} // namespace AdaptiveDB
