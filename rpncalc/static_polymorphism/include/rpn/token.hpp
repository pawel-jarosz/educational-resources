#pragma once

#include <string>
#include <variant>

namespace rpn {

    enum class TokenType {
        NUMBER,
        OPERATOR,
        END
    };

    template<typename T>
    struct Token {
        TokenType type;
        std::variant<T, std::string> value;

        Token(T value) : type(TokenType::NUMBER), value(value) {}
        Token(std::string value) : type(TokenType::OPERATOR), value(std::move(value)) {}
        Token() : type(TokenType::END), value(0.0) {}
    };

}
