#include "ddlsemantic.h"

#include "core/base.h"

#include <functional>

/*
ERROR TYPES:
- [x] Field referenced in foreign key must exist in model
- [ ] Field referenced in foreign key must have same type as primary key
- [ ] Field referenced in foreign key must be primary key
- [x] Model referenced in foreign key must exist
- [x] Every model must have a primary key
- [ ] No duplicate model names
- [ ] No duplicate field names in model
- [ ] Exactly one primary key in model
- [ ] Primary key cannot be nullable
*/

#define BIND_FN(fn) std::bind(&DDLSemanticChecker::fn, this, std::placeholders::_1)

namespace AdaptiveDB
{
    DDLSemanticChecker::DDLSemanticChecker()
    {
    }

    DDLSemanticChecker::~DDLSemanticChecker()
    {
    }

    void DDLSemanticChecker::checkModels(const std::vector<DDLModel> &models)
    {
        m_models = models;
        std::vector<std::function<void(const DDLModel &)>> checks = {
            BIND_FN(fieldReferencedInForeignKeyMustExistInModel),
            BIND_FN(modelMustHavePrimaryKey),
        };

        for (const auto &model : models)
        {
            for (const auto &check : checks)
            {
                check(model);
            }
        }
    }

    void DDLSemanticChecker::fieldReferencedInForeignKeyMustExistInModel(const DDLModel &model)
    {
        for (const auto &field : model.fields)
        {
            // Field that is referenced in a foreign key must exist in the model
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
                            }
                        }
                        if (!fieldFound)
                        {
                            m_errors.push_back(fmt::format("Field {} not found in model {}", field.foreignKey.value().field, field.foreignKey.value().model));
                        }
                    }
                }
                if (!found)
                {
                    m_errors.push_back(fmt::format("Model {} not found referenced by {}.{}", field.foreignKey.value().model , model.name, field.name));
                }
            }
        }
    }

    void DDLSemanticChecker::modelMustHavePrimaryKey(const DDLModel &model)
    {
        bool found = false;
        for (const auto &field : model.fields)
        {
            if (field.primary)
            {
                found = true;
            }
        }
        if (!found)
        {
            m_errors.push_back(fmt::format("Model {} must have a primary key", model.name));
        }
    }
} // namespace AdaptiveDB