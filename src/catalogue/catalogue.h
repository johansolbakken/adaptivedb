#pragma once

/*
## Catalogue

- Metadata about the database
    - Information about tables, columns, views, users
    - Statistics and cost numbers
    - Not directly accessible to the user
        - Through views: SELECT ... FROM INFORMATION_SCHEMA \<view>
- Hot spots
    - Accessed by all queries
    - May use special transaction "tricks" for efficiency
*/

#include "catalogue/basictype.h"

#include <string>
#include <vector>

namespace AdaptiveDB
{
    struct CatalogueColumn
    {
        std::string name;
        BasicType type;
        bool nullable;
    };

    struct CatalogueTable
    {
        std::string name;
        std::vector<CatalogueColumn> columns;
        int primaryKey = -1;
    };

    class Catalogue
    {
    public:
        void addTable(const CatalogueTable &table);
        const std::vector<CatalogueTable> &tables() const { return m_tables; }

    private:
        std::vector<CatalogueTable> m_tables;
    };
}