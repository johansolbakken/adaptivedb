#include "ddl/ddllexer.h"

#include <iostream>

using namespace AdaptiveDB;

std::string tokenTypeToString(TokenType type)
{
    switch (type)
    {
    case TokenType::Model:
        return "Model";
    case TokenType::Identifier:
        return "Identifier";
    case TokenType::OpenBrace:
        return "OpenBrace";
    case TokenType::CloseBrace:
        return "CloseBrace";
    case TokenType::At:
        return "At";
    case TokenType::QuestionMark:
        return "QuestionMark";
    case TokenType::Comma:
        return "Comma";
    case TokenType::OpenParen:
        return "OpenParen";
    case TokenType::CloseParen:
        return "CloseParen";
    case TokenType::String:
        return "String";
    case TokenType::Int:
        return "Int";
    case TokenType::Float:
        return "Float";
    case TokenType::Date:
        return "Date";
    case TokenType::Blob:
        return "Blob";
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