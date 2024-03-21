#pragma once

#include "ddl/ddllexer.h"
#include <vector>
#include <optional>

namespace AdaptiveDB
{
    enum class DDLBasicType
    {
        Int,
        Float,
        Date,
        String,
        Blob
    };

    struct DDLForeignKey
    {
        std::string model;
        std::string field;
    };

    struct DDLField
    {
        DDLBasicType type;
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

    class DDLParser
    {
    public:
        DDLParser(const DDLLexer &lexer);
        DDLModel parseModel();
        std::vector<DDLModel> parseModels();
        bool expect(DDLTokenType type);

    private:
        DDLLexer m_lexer;
        std::vector<DDLToken> m_tokens;
        int m_position = 0;
    };
}