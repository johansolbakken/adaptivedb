#include "ddl/ddllexer.h"

#include <iostream>

using namespace AdaptiveDB;

std::string tokenTypeToString(DDLTokenType type)
{
    switch (type)
    {
    case DDLTokenType::Model:
        return "Model";
    case DDLTokenType::Identifier:
        return "Identifier";
    case DDLTokenType::OpenBrace:
        return "OpenBrace";
    case DDLTokenType::CloseBrace:
        return "CloseBrace";
    case DDLTokenType::At:
        return "At";
    case DDLTokenType::QuestionMark:
        return "QuestionMark";
    case DDLTokenType::Comma:
        return "Comma";
    case DDLTokenType::OpenParen:
        return "OpenParen";
    case DDLTokenType::CloseParen:
        return "CloseParen";
    case DDLTokenType::Type:
        return "Type";
    default:
        return "Unknown";
    }
}

int main()
{
    std::string source = R"(
            model Employee {
                EmployeeID String @id
                FirstName String
                LastName String
                DepartmentID Int?
                JobTitle String?
                HireDate Date
            }

            model Department {
                DepartmentID Int
                DepartmentName String
            }

            model Salary {
                SalaryID Int
                EmployeeID Int @references(Employee, EmployeeID)
                Salary Float
                FromDate Date
                ToDate Date?
            }
)";

    DDLLexer lexer(source);
    auto tokens = lexer.tokenize();
    for (const auto &token : tokens)
    {
        LineColumn lc = positionToLineColumn(source, token.position);
        std::cout << "Token: " << tokenTypeToString(token.type) << " Value: " << token.value << " Line: " << lc.line << " Column: " << lc.column << std::endl;
    }
}