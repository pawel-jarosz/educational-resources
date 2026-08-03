add_library(librpn_static STATIC
    tokenizer.cpp
)
target_include_directories(librpn_static PRIVATE ${Boost_INCLUDE_DIRS})
