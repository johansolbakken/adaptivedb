#include "queryprocessing/ddl/ddllexer.h"

#include "core/base.h"

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
                DepartmentID Int? @references(Department, DepartmentID)
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
    
    if (!lexer.errors().empty())
    {
        for (const auto &error : lexer.errors())
        {
            Log::error(error);
        }
        return 1;
    }
    
    for (const auto &token : tokens)
    {
        LineColumn lc = positionToLineColumn(source, token.position);
        Log::info(fmt::format("Token: {} Value: {} Line: {} Column: {}", tokenTypeToString(token.type), token.value, lc.line, lc.column));
    }
    
    Log::info("No errors found");
}