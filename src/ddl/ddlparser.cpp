#include "ddlparser.h"

namespace AdaptiveDB
{
    DDLParser::DDLParser(const DDLLexer &lexer)
        : m_lexer(lexer)
    {
    }

    Result<DDLModel, ParserError> DDLParser::parseModel()
    {
        if (!expect(DDLTokenType::Model))
        {
            LineColumn lc = positionToLineColumn(m_lexer.source(), m_tokens[m_position].position);
            m_errors.push_back(fmt::format("Expected 'model' at line {}, column {} but got '{}'", lc.line, lc.column, m_tokens[m_position].value));
            return Result<DDLModel, ParserError>::fail(ParserError::ExpectedIdentifier);
        }
        m_position++;

        if (!expect(DDLTokenType::Identifier))
        {
            LineColumn lc = positionToLineColumn(m_lexer.source(), m_tokens[m_position].position);
            m_errors.push_back(fmt::format("Expected identifier at position {} but got '{}'", m_tokens[m_position].position, m_tokens[m_position].value));
            return Result<DDLModel, ParserError>::fail(ParserError::ExpectedIdentifier);
        }
        std::string modelName = m_tokens[m_position].value;
        m_position++;

        if (!expect(DDLTokenType::OpenBrace))
        {
            LineColumn lc = positionToLineColumn(m_lexer.source(), m_tokens[m_position].position);
            m_errors.push_back(fmt::format("Expected '{{' at line {}, column {} but got '{}'", lc.line, lc.column, m_tokens[m_position].value));
            return Result<DDLModel, ParserError>::fail(ParserError::ExpectedIdentifier);
        }
        m_position++;

        DDLModel model;
        model.name = modelName;
        while (m_position < m_tokens.size() && m_tokens[m_position].type != DDLTokenType::CloseBrace)
        {
            DDLField field;
            if (!expect(DDLTokenType::Identifier))
            {
                LineColumn lc = positionToLineColumn(m_lexer.source(), m_tokens[m_position].position);
                m_errors.push_back(fmt::format("Expected identifier at line {}, column {} but got '{}'", lc.line, lc.column, m_tokens[m_position].value));
                return Result<DDLModel, ParserError>::fail(ParserError::ExpectedIdentifier);
            }
            std::string fieldName = m_tokens[m_position].value;
            field.name = fieldName;
            m_position++;

            if (!expect(DDLTokenType::Type))
            {
                LineColumn lc = positionToLineColumn(m_lexer.source(), m_tokens[m_position].position);
                m_errors.push_back(fmt::format("Expected type at line {}, column {} but got '{}'", lc.line, lc.column, m_tokens[m_position].value));
                return Result<DDLModel, ParserError>::fail(ParserError::ExpectedType);
            }
            std::string fieldType = m_tokens[m_position].value;
            m_position++;

            field.type = BasicType::Int;
            if (fieldType == "Int")
            {
                field.type = BasicType::Int;
            }
            else if (fieldType == "Float")
            {
                field.type = BasicType::Float;
            }
            else if (fieldType == "Date")
            {
                field.type = BasicType::Date;
            }
            else if (fieldType == "String")
            {
                field.type = BasicType::String;
            }
            else if (fieldType == "Blob")
            {
                field.type = BasicType::Blob;
            }
            else
            {
                LineColumn lc = positionToLineColumn(m_lexer.source(), m_tokens[m_position].position);
                m_errors.push_back(fmt::format("Unknown type at line {}, column {} but got '{}'", lc.line, lc.column, m_tokens[m_position].value));
                return Result<DDLModel, ParserError>::fail(ParserError::UnknownType);
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
                        LineColumn lc = positionToLineColumn(m_lexer.source(), m_tokens[m_position].position);
                        m_errors.push_back(fmt::format("Expected '(' at line {}, column {} but got '{}'", lc.line, lc.column, m_tokens[m_position].value));
                        return Result<DDLModel, ParserError>::fail(ParserError::ExpectedIdentifier);
                    }
                    m_position++;

                    if (!expect(DDLTokenType::Identifier))
                    {
                        LineColumn lc = positionToLineColumn(m_lexer.source(), m_tokens[m_position].position);
                        m_errors.push_back(fmt::format("Expected identifier at line {}, column {} but got '{}'", lc.line, lc.column, m_tokens[m_position].value));
                        return Result<DDLModel, ParserError>::fail(ParserError::ExpectedIdentifier);
                    }
                    fk.model = m_tokens[m_position].value;
                    m_position++;

                    if (!expect(DDLTokenType::Comma))
                    {
                        LineColumn lc = positionToLineColumn(m_lexer.source(), m_tokens[m_position].position);
                        m_errors.push_back(fmt::format("Expected ',' at line {}, column {} but got '{}'", lc.line, lc.column, m_tokens[m_position].value));
                        return Result<DDLModel, ParserError>::fail(ParserError::ExpectedIdentifier);
                    }
                    m_position++;

                    if (!expect(DDLTokenType::Identifier))
                    {
                        LineColumn lc = positionToLineColumn(m_lexer.source(), m_tokens[m_position].position);
                        m_errors.push_back(fmt::format("Expected identifier at line {}, column {} but got '{}'", lc.line, lc.column, m_tokens[m_position].value));
                        return Result<DDLModel, ParserError>::fail(ParserError::ExpectedIdentifier);
                    }
                    fk.field = m_tokens[m_position].value;
                    m_position++;

                    if (!expect(DDLTokenType::CloseParen))
                    {
                        LineColumn lc = positionToLineColumn(m_lexer.source(), m_tokens[m_position].position);
                        m_errors.push_back(fmt::format("Expected ')' at line {}, column {} but got '{}'", lc.line, lc.column, m_tokens[m_position].value));
                        return Result<DDLModel, ParserError>::fail(ParserError::ExpectedIdentifier);
                    }
                    m_position++;

                    field.foreignKey = fk;
                }
            }

            model.fields.push_back(field);
        }

        if (!expect(DDLTokenType::CloseBrace))
        {
            LineColumn lc = positionToLineColumn(m_lexer.source(), m_tokens[m_position].position);
            m_errors.push_back(fmt::format("Expected '}}' at line {}, column {} but got '{}'", lc.line, lc.column, m_tokens[m_position].value));
            return Result<DDLModel, ParserError>::fail(ParserError::ExpectedIdentifier);
        }

        m_position++;

        return Result<DDLModel, ParserError>::success(model);
    }

    std::vector<DDLModel> DDLParser::parseModels()
    {
        m_tokens = m_lexer.tokenize();
        std::vector<DDLModel> models;
        while (m_position < m_tokens.size())
        {
            auto model = parseModel();
            if (model) {
                models.push_back(model);
            } else {
                return models;
            }
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