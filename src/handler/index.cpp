#include "index.h"

#include "core/base.h"

namespace AdaptiveDB
{
    void index(Request &req, nlohmann::json &res)
    {
        res["body"] = req.body;
    }
}