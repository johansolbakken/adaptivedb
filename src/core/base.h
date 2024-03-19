#pragma once

#include <memory>

#include "core/log.h"

namespace AdaptiveDB
{
    template <typename T>
    using Ref = std::shared_ptr<T>;

    template <typename T, typename... Args>
    constexpr Ref<T> createRef(Args &&...args)
    {
        return std::make_shared<T>(std::forward<Args>(args)...);
    }

    struct VersionConfig {
        const int major;
        const int minor;
        const int patch;
    };
    
    VersionConfig versionConfig();
} // namespace AdaptiveDB