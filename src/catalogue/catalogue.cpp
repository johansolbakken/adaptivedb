#include "catalogue.h"

#include <filesystem>
#include <fstream>

#include <nlohmann/json.hpp>

namespace AdaptiveDB
{
	static std::string persistancePath = ".adaptiveDB/catalogue.json";

	Catalogue::Catalogue()
	{
		if (!std::filesystem::exists(".adaptiveDB"))
		{
			return;
		}

		std::ifstream file(persistancePath);
		nlohmann::json json;
		file >> json;

		for (const auto& table: json)
		{
			CatalogueTable catalogueTable;
			catalogueTable.name = table["name"];
			catalogueTable.primaryKey = table["primaryKey"];

			for (const auto& column: table["columns"])
			{
				CatalogueColumn catalogueColumn;
				catalogueColumn.name = column["name"];
				catalogueColumn.type = static_cast<BasicType>(column["type"]);
				catalogueColumn.nullable = column["nullable"];
				catalogueTable.columns.push_back(catalogueColumn);
			}

			m_tables.push_back(catalogueTable);
		}
	}

	Catalogue::~Catalogue()
	{
		if (!std::filesystem::exists(".adaptiveDB"))
		{
			std::filesystem::create_directory(".adaptiveDB");
		}

		nlohmann::json json = nlohmann::json::array();

		for (const auto& table: m_tables)
		{
			nlohmann::json jsonTable;
			jsonTable["name"] = table.name;
			jsonTable["primaryKey"] = table.primaryKey;

			nlohmann::json jsonColumns = nlohmann::json::array();
			for (const auto& column: table.columns)
			{
				nlohmann::json jsonColumn;
				jsonColumn["name"] = column.name;
				jsonColumn["type"] = static_cast<int>(column.type);
				jsonColumn["nullable"] = column.nullable;
				jsonColumns.push_back(jsonColumn);
			}

			jsonTable["columns"] = jsonColumns;
			json.push_back(jsonTable);
		}

		std::ofstream file(persistancePath);
		file << json;
	}

	void Catalogue::addTable(const CatalogueTable& table)
	{
		m_tables.push_back(table);
	}
}