#include "ddlsemantic.h"

#include "core/base.h"

/*
ERROR TYPES:
- Field referenced in foreign key must exist in model
- Field referenced in foreign key must have same type as primary key
- Field referenced in foreign key must be primary key
- Model referenced in foreign key must exist
- Every model must have a primary key
- No duplicate model names
- No duplicate field names in model
- Exactly one primary key in model
- Primary key cannot be nullable
*/

namespace AdaptiveDB
{
    DDLSemanticChecker::DDLSemanticChecker()
    {
    }

    DDLSemanticChecker::~DDLSemanticChecker()
    {
    }

    void DDLSemanticChecker::checkModel(const DDLModel &model)
    {
        // For every reference in the model check if the model exists and the field exists and the field is of the correct type
        for (const auto &field : model.fields)
        {
            if (field.foreignKey.has_value())
            {
                bool found = false;
                for (const auto &m : m_models)
                {
                    if (m.name == field.foreignKey.value().model)
                    {
                        found = true;
                        bool fieldFound = false;
                        for (const auto &f : m.fields)
                        {
                            if (f.name == field.foreignKey.value().field)
                            {
                                fieldFound = true;
                                if (f.type != field.type)
                                {
                                    Log::error(fmt::format("Field {} in model {} must have same type as primary key", field.foreignKey.value().field, field.foreignKey.value().model));
                                }

                                // primary key
                                if (!f.primary)
                                {
                                    Log::error(fmt::format("Field {} in model {} must be primary key", field.foreignKey.value().field, field.foreignKey.value().model));
                                }
                            }
                        }
                        if (!fieldFound)
                        {
                            Log::error(fmt::format("Field {} not found in model {}", field.foreignKey.value().field, field.foreignKey.value().model));
                        }
                    }
                }
                if (!found)
                {
                    Log::error(fmt::format("Model {} not found referenced by {}.{}", field.foreignKey.value().model , model.name, field.name));
                }
            }
        }
    }

    void DDLSemanticChecker::checkModels(const std::vector<DDLModel> &models)
    {
        m_models = models;
        for (const auto &model : models)
        {
            checkModel(model);
        }
    }
} // namespace AdaptiveDB