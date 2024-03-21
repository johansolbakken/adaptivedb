#pragma once

#include "ddl/ddlparser.h"

namespace AdaptiveDB {
    class DDLSemanticChecker {
    public:
        DDLSemanticChecker();
        ~DDLSemanticChecker();

        void checkModel(const DDLModel &model);
        void checkModels(const std::vector<DDLModel> &models);

    private:
        std::vector<DDLModel> m_models;
    };
}