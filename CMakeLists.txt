cmake_minimum_required(VERSION 3.5)

project(AdaptiveDB VERSION 0.0.1 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Assuming build directory next to source directory
configure_file(src/core/versionconfig.h.in ${CMAKE_CURRENT_SOURCE_DIR}/src/core/versionconfig.h)

file(GLOB_RECURSE SOURCES "src/**.cpp")

add_subdirectory(vendor/json)

add_executable(AdaptiveDB ${SOURCES})
target_include_directories(AdaptiveDB PRIVATE "src" "vendor/json/include")
target_link_libraries(AdaptiveDB PRIVATE nlohmann_json::nlohmann_json)