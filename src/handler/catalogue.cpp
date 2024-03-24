#include "catalogue.h"

#include "catalogue/basictype.h"
#include "core/application.h"
#include "catalogue/catalogue.h"

#include "queryprocessing/ddlsemantic.h"

namespace AdaptiveDB
{
    void getAllTables(Request &req, nlohmann::json &res) {
        auto catalogue = Application::instance().catalogue();
        res["tables"] = nlohmann::json::array();
        for (const auto &table : catalogue->tables()) {
            nlohmann::json tableJson;
            tableJson["name"] = table.name;
            nlohmann::json columnsJson;
            for (const auto &column : table.columns) {
                nlohmann::json columnJson;
                columnJson["name"] = column.name;
                columnJson["type"] = basicTypeStrings[column.type];
                columnJson["nullable"] = column.nullable;
                if (table.primaryKey == &column - &table.columns[0]) {
                    columnJson["primary"] = true;
                }
                columnsJson.push_back(columnJson);
            }
            tableJson["columns"] = columnsJson;
            res["tables"].push_back(tableJson);
        }
    }

    void createTableBySchema(Request &req, nlohmann::json &res)
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

        for (const auto &model : models)
        {
            CatalogueTable table;
            table.name = model.name;
            int i = 0;
            for (const auto &field : model.fields)
            {
                CatalogueColumn tableColumn;
                tableColumn.name = field.name;
                tableColumn.type = field.type;
                tableColumn.nullable = field.nullable;
                if (field.primary)
                {
                    table.primaryKey = i;
                }
                table.columns.push_back(tableColumn);
                i++;
            }
            Application::instance().catalogue()->addTable(table);
        }


        res["status"] = "OK";
        res["message"] = "Tables created";
    }
}