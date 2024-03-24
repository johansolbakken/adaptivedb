
add_executable(ddllexertest queryprocessing/ddl/ddllexertest.cpp)
target_link_libraries(ddllexertest PUBLIC adbcore)
target_include_directories(ddllexertest PUBLIC ${ADB_INCLUDE_DIRS})

add_executable(ddlparsertest queryprocessing/ddl/ddlparsertest.cpp)
target_link_libraries(ddlparsertest PUBLIC adbcore)
target_include_directories(ddlparsertest PUBLIC ${ADB_INCLUDE_DIRS})

add_executable(ddlsemantictest queryprocessing/ddl/ddlsemantictest.cpp)
target_link_libraries(ddlsemantictest PUBLIC adbcore)
target_include_directories(ddlsemantictest PUBLIC ${ADB_INCLUDE_DIRS})