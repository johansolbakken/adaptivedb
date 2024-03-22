#pragma once

#include "core/server.h"

namespace AdaptiveDB {
    void getAllTables(Request &req, nlohmann::json &res);
    void createTableBySchema(Request &req, nlohmann::json &res);
}