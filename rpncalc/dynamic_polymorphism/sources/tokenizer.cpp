#include <rpn/tokenizer.hpp>

#include <string>
#include <sstream>

#include <boost/lexical_cast.hpp>

namespace rpn {

Tokenizer::Tokenizer(std::string input)
    : ss_(std::move(input)){

}

void Tokenizer::reset(std::string input){
    std::stringstream new_input{std::move(input)};
    ss_.swap(new_input);
}

Token Tokenizer::next() {
    std::string element;
    ss_ >> element;
    try {
        double result = boost::lexical_cast<double>(element);
        return Token {
            Token::TokenType::NUMBER,
            result
        };
    }
    catch (boost::bad_lexical_cast& err) {
        if (not element.empty()) {
            return Token {
                Token::TokenType::OPERATOR,
                element
            };
        }
    }
    return Token {
    Token::TokenType::END,
        0.0};
}


}
