#include <rpn/stack.hpp>
#include <gtest/gtest.h>

namespace rpn::tests {
    class DoubleStackStdWrapperTestImpl : public DoubleStackStdWrapper {
    public:
        std::stack<double>& get_stack() {
            return DoubleStackStdWrapper::stack_;
        }
    };

    TEST(StackWrapperTests, VerifyBasicCapabilities)
    {
        DoubleStackStdWrapperTestImpl stack;
        double expected_value = 2.0;
        EXPECT_TRUE(stack.empty());
        stack.push(expected_value);
        EXPECT_FALSE(stack.empty());
        EXPECT_EQ(stack.get_stack().top(), expected_value);
        stack.pop();
        EXPECT_TRUE(stack.empty());
    }

}
