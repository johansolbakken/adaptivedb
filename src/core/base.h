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

    // Result type for functions that can fail
    template <typename T, typename E>
    struct Result
    {
        T value;
        bool m_success;
        E error;

        operator bool() const { return m_success; }
        operator T() const { return value; }
        operator E() const { return error; }

        static Result<T, E> success(T value)
        {
            return {value, true, E()};
        }

        static Result<T, E> fail(E error)
        {
            return {T(), false, error};
        }
    };
} // namespace AdaptiveDB