#include "ddl/ddlparser.h"

using namespace AdaptiveDB;

int main()
{
    std::string source = R"(
            model Employee {
                EmployeeID String @id
                FirstName String
                LastName String
                DepartmentID Int? @references(Department, DepartmentID)
                JobTitle String?
                HireDate Date
            }

            model Department {
                DepartmentID Int @id
                DepartmentName String
            }

            model Salary {
                SalaryID Int @id
                EmployeeID Int @references(Employee, EmployeeID)
                Salary Float
                FromDate Date
                ToDate Date?
            }
)";

    DDLLexer lexer(source);
    DDLParser parser(lexer);
    auto models = parser.parseModels();

    if (parser.errors().size() > 0)
    {
        for (auto &error : parser.errors())
        {
            Log::error(error);
        }
        return 1;
    }

    for (auto &model : models)
    {
        Log::info(fmt::format("Model: {}", model.name));
        for (auto &field : model.fields)
        {
            Log::info(fmt::format("  Field: {}", field.name));
            std::string type;

            switch (field.type)
            {
            case DDLBasicType::Int:
                type = "Int";
                break;
            case DDLBasicType::Float:

                type = "Float";
                break;
            case DDLBasicType::Date:
                type = "Date";
                break;
            case DDLBasicType::String:
                type = "String";
                break;
            case DDLBasicType::Blob:
                type = "Blob";
                break;
            default:
                type = "Unknown";
                break;
            }

            Log::info(fmt::format("    Type: {}", type));
            Log::info(fmt::format("    Nullable: {}", field.nullable ? "true" : "false"));
            Log::info(fmt::format("    Primary: {}", field.primary ? "true" : "false"));

            if (field.foreignKey)
            {
                Log::info(fmt::format("    Foreign Key: {}.{}", field.foreignKey->model, field.foreignKey->field));
            }
        }
    }
}