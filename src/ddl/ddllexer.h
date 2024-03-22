#pragma once

#include <string>
#include <vector>

namespace AdaptiveDB
{
    enum class DDLTokenType
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
        Type
    };

    struct DDLToken
    {
        DDLTokenType type;
        std::string value;
        int position;
    };

    class DDLLexer
    {
    public:
        DDLLexer(const std::string &source);
        ~DDLLexer();

        std::vector<DDLToken> tokenize();
        const std::vector<std::string>& errors() const { return m_errors; }
        const std::string& source() const { return m_source; }

    private:
        std::string m_source;
        int m_position;
        std::vector<std::string> m_errors;
    };

    struct LineColumn
    {
        int line;
        int column;
    };

    LineColumn positionToLineColumn(const std::string &source, int position);
} // namespace AdaptiveDB