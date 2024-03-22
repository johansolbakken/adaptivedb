#pragma once

#include <string>
#include <map>

namespace AdaptiveDB
{
	enum class BasicType
	{
		Int,
		Float,
		Date,
		String,
		Blob
	};

	static std::map<BasicType, std::string> basicTypeStrings = {
			{ BasicType::Int,    "Int" },
			{ BasicType::Float,  "Float" },
			{ BasicType::Date,   "Date" },
			{ BasicType::String, "String" },
			{ BasicType::Blob,   "Blob" }
	};
} // namespace AdaptiveDB