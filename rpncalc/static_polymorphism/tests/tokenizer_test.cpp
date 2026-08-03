
#include "rpn/tokenizer.hpp"

#include <rpn/stack.hpp>
#include <gtest/gtest.h>

namespace rpn::tests {

TEST(RegularArithmeticTokenizerTest, CheckNumbersSplittedBySpace) {
    std::vector<double> expected_values {1.2, 2.3, 3.4};
    std::stringstream ss;
    for (const auto& value: expected_values) {
        ss << " " << value;
    }
    RegularArithmeticTokenizer tokenizer{ss.str()};
    for (const auto& value: expected_values) {
        const auto [type, variants] = tokenizer.next();
        EXPECT_EQ(type, TokenType::NUMBER);
        EXPECT_EQ(value, std::get<double>(variants));
    }
}

TEST(RegularArithmeticTokenizerTest, CheckEndToken) {
    RegularArithmeticTokenizer tokenizer{};
    auto token = tokenizer.next();
    EXPECT_EQ(token.type, TokenType::END);
    tokenizer.reset("2 3");
    (void)tokenizer.next();
    (void)tokenizer.next();
    token = tokenizer.next();
    EXPECT_EQ(token.type, TokenType::END);
}

TEST(RegularArithmeticTokenizerTest, CheckActionToken) {
    RegularArithmeticTokenizer tokenizer{"+ - sin cos"};
    std::vector<std::string> tokens{"+", "-", "sin", "cos"};
    for (const auto& expected_token: tokens) {
        auto [token_type, value] = tokenizer.next();
        EXPECT_EQ(token_type, TokenType::OPERATOR);
        EXPECT_EQ(expected_token, std::get<std::string>(value));
    }
}

}
