#pragma once

#include <string>
#include <vector>

namespace AdaptiveDB
{
    enum class TokenType
    {
        Model,
        Identifier,
        OpenBrace,
        CloseBrace,
        At,
        QuestionMark,
        Comma,
        OpenParen,
        CloseParen,

        // Types
        String,
        Int,
        Float,
        Date,
        Blob,
    };

    struct DDLToken
    {
        TokenType type;
        std::string value;
        int position;
    };

    class DDLLexer
    {
    public:
        DDLLexer(const std::string &source);
        ~DDLLexer();

        std::vector<DDLToken> tokenize();

    private:
        std::string m_source;
        int m_position;
    };

    struct LineColumn
    {
        int line;
        int column;
    };

    LineColumn positionToLineColumn(const std::string &source, int position);
} // namespace AdaptiveDB