cmake_minimum_required(VERSION 3.5)

project(AdaptiveDB)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

file(GLOB_RECURSE SOURCES "src/**.cpp")

add_executable(AdaptiveDB ${SOURCES})
target_include_directories(AdaptiveDB PRIVATE "src")