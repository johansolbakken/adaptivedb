#pragma once

#include "queryprocessing/ddlparser.h"

namespace AdaptiveDB
{
    class DDLSemanticChecker
    {
    public:
        DDLSemanticChecker();
        ~DDLSemanticChecker();

        void checkModels(const std::vector<DDLModel> &models);

        [[nodiscard]] const std::vector<std::string> &errors() const { return m_errors; }

    private: // Checks
        void fieldReferencedInForeignKeyMustExistInModel(const DDLModel &model);
        void modelMustHavePrimaryKey(const DDLModel &model);
        

    private:
        std::vector<DDLModel> m_models;
        std::vector<std::string> m_errors;
    };
}