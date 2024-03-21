#include "ddl/ddlsemantic.h"

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
                // reference to non-existing model
                NonExisting Int @references(NonExistingModel, NonExistingField)
            }
)";

    DDLLexer lexer(source);
    DDLParser parser(lexer);
    auto models = parser.parseModels();

    // Should show error message because EmployeeID is referenced as Int but is String
    DDLSemanticChecker checker;
    checker.checkModels(models);
}