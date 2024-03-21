
add_executable(ddllexertest ddl/ddllexertest.cpp)
target_link_libraries(ddllexertest PRIVATE adbcore)
target_include_directories(ddllexertest PRIVATE ${ADB_INCLUDE_DIRS})

add_executable(ddlparsertest ddl/ddlparsertest.cpp)
target_link_libraries(ddlparsertest PRIVATE adbcore)
target_include_directories(ddlparsertest PRIVATE ${ADB_INCLUDE_DIRS})