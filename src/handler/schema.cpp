#include "schema.h"

#include "core/base.h"

#include "queryprocessing/ddlsemantic.h"

namespace AdaptiveDB
{
    void schema(Request &req, nlohmann::json &res)
    {
        auto body = req.body;
        if (!body.contains("schema"))
        {
            res["status"] = "ERROR";
            res["message"] = "Body requires a 'schema' field";
            return;
        }

        DDLLexer lexer(body["schema"]);
        auto tokens = lexer.tokenize();

        if (lexer.errors().size() > 0)
        {
            nlohmann::json errors;
            for (auto &error : lexer.errors())
            {
                errors.push_back(error);
            }
            res["status"] = "ERROR";
            res["message"] = "Lexer error";
            res["errors"] = errors;
            return;
        }

        lexer = DDLLexer(body["schema"]);
        DDLParser parser(lexer);
        auto models = parser.parseModels();
        
        if (parser.errors().size() > 0)
        {
            nlohmann::json errors;
            for (auto &error : parser.errors())
            {
                errors.push_back(error);
            }
            res["status"] = "ERROR";
            res["message"] = "Parser error";
            res["errors"] = errors;
            return;
        }

        DDLSemanticChecker checker;
        checker.checkModels(models);

        if (checker.errors().size() > 0)
        {
            nlohmann::json errors;
            for (auto &error : checker.errors())
            {
                errors.push_back(error);
            }
            res["status"] = "ERROR";
            res["message"] = "Semantic error";
            res["errors"] = errors;
            return;
        }



        res["status"] = "OK";
        res["message"] = "No errors found";
    }
}