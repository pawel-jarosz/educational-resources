#include <gtest/gtest.h>
#include <gmock/gmock.h>

#include <utility>

#include <rpn/computation_strategy.hpp>

#include "mocks/strategy_mock.hpp"

namespace rpn::tests {

TEST(PlusStrategyTest, SuccessStory) {
    PlusStrategy plus;
    EXPECT_EQ(plus.get_operator(), "+");
    DoubleStackStdWrapper double_stack;
    double_stack.push(2.0);
    double_stack.push(3.0);
    plus.call(double_stack);
    EXPECT_EQ(double_stack.top(), 5.0);
}

TEST(MinusStrategyTest, SuccessStory) {
    MinusStrategy minus;
    EXPECT_EQ(minus.get_operator(), "-");
    DoubleStackStdWrapper double_stack;
    double_stack.push(2.0);
    double_stack.push(3.0);
    minus.call(double_stack);
    EXPECT_EQ(double_stack.top(), -1.0);
}

TEST(MultStrategyTest, SuccessStory) {
    MultStrategy mult;
    EXPECT_EQ(mult.get_operator(), "*");
    DoubleStackStdWrapper double_stack;
    double_stack.push(2.0);
    double_stack.push(3.0);
    mult.call(double_stack);
    EXPECT_EQ(double_stack.top(), 6.0);
}

TEST(DivStrategyTest, SuccessStory) {
    DivStrategy div;
    EXPECT_EQ(div.get_operator(), "/");
    DoubleStackStdWrapper double_stack;
    double_stack.push(6.0);
    double_stack.push(3.0);
    div.call(double_stack);
    EXPECT_EQ(double_stack.top(), 2.0);
}

TEST(StrategySelectorTest, SuccessStory) {
    using ::testing::_;

    auto strategy = std::make_unique<mocks::StrategyMock>("dummy");
    DoubleStackStdWrapper double_stack;
    EXPECT_CALL(*strategy, call(_)).Times(1);
    ComputationStrategySelector selector;
    selector.register_strategy(std::move(strategy));
    EXPECT_NO_THROW({
        auto& result = selector.get_strategy("dummy");
        result.call(double_stack);
    });
}

}


