add_library(librpn stack.cpp tokenizer.cpp)
target_include_directories(librpn PRIVATE ${Boost_INCLUDE_DIRS})
