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
    };

    struct CatalogueTable
    {
        std::string name;
        std::vector<CatalogueColumn> columns;
    };

    class Catalogue
    {
    public:
    private:
        std::vector<CatalogueTable> m_tables;
    };
}