
add_executable(ddllexertest ddl/ddllexertest.cpp ../src/ddl/ddllexer.cpp)
target_link_libraries(ddllexertest PRIVATE adbcore)
target_include_directories(ddllexertest PRIVATE ${ADB_INCLUDE_DIRS})