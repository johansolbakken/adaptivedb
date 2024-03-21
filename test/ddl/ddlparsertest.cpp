#include "ddl/ddlparser.h"

#include <iostream>

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
    for (auto &model : models)
    {
        std::cout << "Model: " << model.name << std::endl;
        for (auto &field : model.fields)
        {
            std::cout << "  Field: " << field.name << std::endl;
            std::cout << "    Type: ";
            switch (field.type)
            {
            case DDLBasicType::Int:
                std::cout << "Int";
                break;
            case DDLBasicType::Float:

                std::cout << "Float";
                break;
            case DDLBasicType::Date:
                std::cout << "Date";
                break;
            case DDLBasicType::String:
                std::cout << "String";
                break;
            case DDLBasicType::Blob:
                std::cout << "Blob";
                break;
            default:
                std::cout << "Unknown";
                break;
            }

            std::cout << std::endl;
            std::cout << "    Nullable: " << (field.nullable ? "true" : "false") << std::endl;
            std::cout << "    Primary: " << (field.primary ? "true" : "false") << std::endl;

            if (field.foreignKey)
            {
                std::cout << "    Foreign Key: " << field.foreignKey->model << "." << field.foreignKey->field << std::endl;
            }
        }
    }
}