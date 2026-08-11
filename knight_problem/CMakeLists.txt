cmake_minimum_required(VERSION 3.31)
project(knight_problem C)

set(CMAKE_C_STANDARD 17)
set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -Wpedantic -Werror")

add_subdirectory(src)
