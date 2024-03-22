#include "catalogue.h"

namespace AdaptiveDB
{
    void Catalogue::addTable(const CatalogueTable &table)
    {
        m_tables.push_back(table);
    }
}