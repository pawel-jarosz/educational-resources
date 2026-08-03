#include <rpn/calculator.hpp>

namespace rpn {
    double Calculator::compute(ITokenizer &tokenizer) {
        Token token {
            Token::TokenType::END, 0.0
        };
        do {
            token = tokenizer.next();
            switch (token.type) {
                case Token::TokenType::END: break;
                case Token::TokenType::NUMBER: double_stack_.push(std::get<double>(token.value));
                    break;
                case Token::TokenType::OPERATOR:
                    selector_.get_strategy(std::get<std::string>(token.value)).call(double_stack_);
                    break;
            }
        } while (token.type != Token::TokenType::END);
        return double_stack_.top();
    }

}
