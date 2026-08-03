add_library(librpn stack.cpp tokenizer.cpp computation_strategy.cpp calculator.cpp)
target_include_directories(librpn PRIVATE ${Boost_INCLUDE_DIRS})

add_executable(dynamic_rpncalc dynamic_main.cpp)
target_link_libraries(dynamic_rpncalc PRIVATE librpn)

