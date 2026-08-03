enable_testing()

add_executable(rpn_test stack_wrapper_test.cpp tokenizer_test.cpp computation_strategy_test.cpp)
target_link_libraries(rpn_test librpn GTest::gmock_main)

include(GoogleTest)
gtest_discover_tests(rpn_test)
