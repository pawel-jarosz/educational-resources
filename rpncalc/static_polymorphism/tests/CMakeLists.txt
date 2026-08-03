add_executable(rpn_static_test tokenizer_test.cpp)
target_link_libraries(rpn_static_test librpn_static GTest::gmock_main)

include(GoogleTest)
gtest_discover_tests(rpn_static_test)
