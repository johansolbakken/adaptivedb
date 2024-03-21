#include "ddlparser.h"

#include "core/log.h"

namespace AdaptiveDB
{
    DDLParser::DDLParser(const DDLLexer &lexer)
        : m_lexer(lexer)
    {
    }

    DDLModel DDLParser::parseModel()
    {
        if (!expect(DDLTokenType::Model))
        {
            Log::info(fmt::format("{}", m_tokens[m_position].value));
            throw std::runtime_error("Expected model");
        }
        m_position++;

        if (!expect(DDLTokenType::Identifier))
        {
            throw std::runtime_error("1Expected identifier");
        }
        std::string modelName = m_tokens[m_position].value;
        m_position++;

        if (!expect(DDLTokenType::OpenBrace))
        {
            throw std::runtime_error("Expected {");
        }
        m_position++;

        DDLModel model;
        model.name = modelName;
        while (m_position < m_tokens.size() && m_tokens[m_position].type != DDLTokenType::CloseBrace)
        {
            DDLField field;
            if (!expect(DDLTokenType::Identifier))
            {
                Log::info(fmt::format("{}", m_tokens[m_position].value));
                throw std::runtime_error("2Expected identifier");
            }
            std::string fieldName = m_tokens[m_position].value;
            field.name = fieldName;
            m_position++;

            if (!expect(DDLTokenType::Type))
            {
                throw std::runtime_error("3Expected type");
            }
            std::string fieldType = m_tokens[m_position].value;
            m_position++;

            field.type = DDLBasicType::Int;
            if (fieldType == "Int")
            {
                field.type = DDLBasicType::Int;
            }
            else if (fieldType == "Float")
            {
                field.type = DDLBasicType::Float;
            }
            else if (fieldType == "Date")
            {
                field.type = DDLBasicType::Date;
            }
            else if (fieldType == "String")
            {
                field.type = DDLBasicType::String;
            }
            else if (fieldType == "Blob")
            {
                field.type = DDLBasicType::Blob;
            }
            else
            {
                throw std::runtime_error("Unknown type");
            }

            field.nullable = false;
            if (m_position < m_tokens.size() && m_tokens[m_position].type == DDLTokenType::QuestionMark)
            {
                field.nullable = true;
                m_position++;
            }

            field.primary = false;
            while (m_position < m_tokens.size() && m_tokens[m_position].type == DDLTokenType::At)
            {
                m_position++;

                if (m_position >= m_tokens.size())
                {
                    break;
                }

                if (m_tokens[m_position].type == DDLTokenType::Identifier && m_tokens[m_position].value == "id")
                {
                    field.primary = true;
                    m_position++;
                }

                // Foreign keys
                //@references(Employee, EmployeeID)
                if (m_tokens[m_position].type == DDLTokenType::Identifier && m_tokens[m_position].value == "references")
                {
                    DDLForeignKey fk;
                    m_position++;
                    if (!expect(DDLTokenType::OpenParen))
                    {
                        throw std::runtime_error("Expected (");
                    }
                    m_position++;

                    if (!expect(DDLTokenType::Identifier))
                    {
                        throw std::runtime_error("Expected identifier");
                    }
                    fk.model = m_tokens[m_position].value;
                    m_position++;

                    if (!expect(DDLTokenType::Comma))
                    {
                        throw std::runtime_error("Expected ,");
                    }
                    m_position++;

                    if (!expect(DDLTokenType::Identifier))
                    {
                        throw std::runtime_error("Expected identifier");
                    }
                    fk.field = m_tokens[m_position].value;
                    m_position++;

                    if (!expect(DDLTokenType::CloseParen))
                    {
                        throw std::runtime_error("Expected )");
                    }
                    m_position++;

                    field.foreignKey = fk;
                }

            }

            model.fields.push_back(field);
        }

        if (!expect(DDLTokenType::CloseBrace))
        {
            throw std::runtime_error("Expected }");
        }

        m_position++;

        return model;
    }

    std::vector<DDLModel> DDLParser::parseModels()
    {
        m_tokens = m_lexer.tokenize();
        std::vector<DDLModel> models;
        while (m_position < m_tokens.size())
        {
            models.push_back(parseModel());
        }
        return models;
    }

    bool DDLParser::expect(DDLTokenType type)
    {
        if (m_position >= m_tokens.size())
        {
            return false;
        }

        if (m_tokens[m_position].type != type)
        {
            return false;
        }
        return true;
    }
}