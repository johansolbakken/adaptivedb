#include "base.h"
#include "core/versionconfig.h"

namespace AdaptiveDB
{

    VersionConfig versionConfig()
    {
        return VersionConfig{AdaptiveDB_VERSION_MAJOR, AdaptiveDB_VERSION_MINOR, AdaptiveDB_VERSION_PATCH};
    }

} // namespace AdaptiveDB