#pragma once

#include <fmt/color.h>

struct LogColor
{
    static constexpr auto info = fmt::fg(fmt::color::green);
    static constexpr auto warn = fmt::fg(fmt::color::yellow);
    static constexpr auto error = fmt::fg(fmt::color::red);
};