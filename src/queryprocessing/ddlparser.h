#pragma once

#include "core/base.h"
#include "queryprocessing/ddllexer.h"
#include "catalogue/catalogue.h"

#include <vector>
#include <optional>

namespace AdaptiveDB
{
    struct DDLForeignKey
    {
        std::string model;
        std::string field;
    };

    struct DDLField
    {
        BasicType type;
        std::string name;
        bool nullable;
        // bool unique; TODO: Add @unique to DDL
        bool primary;
        std::optional<DDLForeignKey> foreignKey;
    };

    struct DDLModel
    {
        std::string name;
        std::vector<DDLField> fields;
    };

    enum class ParserError
    {
        ExpectedIdentifier,
        ExpectedType,
        UnknownType
    };

    class DDLParser
    {
    public:
        DDLParser(const DDLLexer &lexer);
        Result<DDLModel, ParserError> parseModel();
        std::vector<DDLModel> parseModels();
        bool expect(DDLTokenType type);

        const std::vector<std::string> &errors() const { return m_errors; }

    private:
        DDLLexer m_lexer;
        std::vector<DDLToken> m_tokens;
        int m_position = 0;
        std::vector<std::string> m_errors;
    };
}