#include "index.h"

#include "core/base.h"

namespace AdaptiveDB
{
    void index(Request &req, nlohmann::json &res)
    {
        if (!req.body.contains("operator"))
        {
            res["error"] = "No operator provided";
            return;
        }

        if (!req.body.contains("operands"))
        {
            res["error"] = "No operands provided";
            return;
        }

        auto operator_ = req.body["operator"].get<std::string>();
        auto operands = req.body["operands"].get<std::vector<int>>();

        if (operator_ == "add")
        {
            int result = 0;
            for (auto operand : operands)
            {
                result += operand;
            }
            res["result"] = result;
        }
        else if (operator_ == "multiply")
        {
            int result = 1;
            for (auto operand : operands)
            {
                result *= operand;
            }
            res["result"] = result;
        }
        else
        {
            res["error"] = "Invalid operator";
        }
    }
}