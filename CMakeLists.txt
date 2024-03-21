cmake_minimum_required(VERSION 3.5)

project(AdaptiveDB VERSION 0.0.1 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Assuming build directory next to source directory
configure_file(src/core/versionconfig.h.in ${CMAKE_CURRENT_SOURCE_DIR}/src/core/versionconfig.h)

file(GLOB_RECURSE SOURCES "src/**.cpp")
list(REMOVE_ITEM SOURCES "${CMAKE_CURRENT_SOURCE_DIR}/src/core/entrypoint.cpp")

add_subdirectory(vendor/json)
add_subdirectory(vendor/fmt)

set(ADB_INCLUDE_DIRS "${CMAKE_CURRENT_SOURCE_DIR}/src" "${CMAKE_CURRENT_SOURCE_DIR}/vendor/json/include" "${CMAKE_CURRENT_SOURCE_DIR}/vendor/fmt/include" CACHE INTERNAL "ADB include directories")

add_library(adbcore STATIC ${SOURCES})
target_include_directories(adbcore PUBLIC ${ADB_INCLUDE_DIRS})
target_link_libraries(adbcore PUBLIC fmt::fmt nlohmann_json::nlohmann_json)

add_executable(AdaptiveDB src/core/entrypoint.cpp)
target_include_directories(AdaptiveDB PUBLIC ${ADB_INCLUDE_DIRS})
target_link_libraries(AdaptiveDB PUBLIC adbcore)

add_subdirectory(test)