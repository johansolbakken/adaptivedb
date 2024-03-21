#include "ddllexer.h"

#include <map>
#include <sstream>

#include "core/log.h"

namespace AdaptiveDB
{
    DDLLexer::DDLLexer(const std::string &source)
        : m_source(source), m_position(0)
    {
    }

    DDLLexer::~DDLLexer()
    {
    }

    std::vector<DDLToken> DDLLexer::tokenize()
    {
        std::vector<DDLToken> tokens;
        std::istringstream stream(m_source);
        std::string token;
        char ch;

        auto addToken = [&](TokenType type, const std::string &value)
        {
            tokens.push_back({type, value, m_position});
        };

        while (m_position < m_source.length())
        {
            ch = m_source[m_position];

            // Skip whitespace
            if (std::isspace(ch))
            {
                ++m_position;
                continue;
            }

            if (std::isalpha(ch))
            { // Start of a keyword or identifier
                size_t start = m_position;
                while (std::isalnum(m_source[m_position]) || m_source[m_position] == '_')
                    ++m_position;
                std::string value = m_source.substr(start, m_position - start);

                // Match keywords or types
                static const std::map<std::string, TokenType> keywords = {
                    {"model", TokenType::Model},
                    {"String", TokenType::String},
                    {"Int", TokenType::Int},
                    {"Float", TokenType::Float},
                    {"Date", TokenType::Date},
                    // Add other keywords and types as needed
                };

                auto it = keywords.find(value);
                if (it != keywords.end())
                {
                    addToken(it->second, value);
                }
                else
                {
                    addToken(TokenType::Identifier, value);
                }
            }
            else if (ch == '?')
            {
                addToken(TokenType::QuestionMark, "?");
                ++m_position;
            }
            else if (ch == '{')
            {
                addToken(TokenType::OpenBrace, "{");
                ++m_position;
            }
            else if (ch == '}')
            {
                addToken(TokenType::CloseBrace, "}");
                ++m_position;
            }
            else if (ch == '(')
            {
                addToken(TokenType::OpenParen, "(");
                ++m_position;
            }
            else if (ch == ')')
            {
                addToken(TokenType::CloseParen, ")");
                ++m_position;
            }
            else if (ch == '@')
            {
                addToken(TokenType::At, "@");
                ++m_position;
            }
            else if (ch == ',')
            {
                addToken(TokenType::Comma, ",");
                ++m_position;
            }
            else
            {
                // Handle unexpected character or implement additional logic for other characters
                LineColumn lc = positionToLineColumn(m_source, m_position);
                Log::error(fmt::format("Unexpected character '{}' at line {}, column {}", ch, lc.line, lc.column));
                ++m_position;
            }
        }

        return tokens;
    }

    LineColumn positionToLineColumn(const std::string &source, int position)
    {
        LineColumn lc = {1, 1};
        for (int i = 0; i < position; ++i)
        {
            if (source[i] == '\n')
            {
                ++lc.line;
                lc.column = 1;
            }
            else
            {
                ++lc.column;
            }
        }
        return lc;
    }

} // namespace AdaptiveDB