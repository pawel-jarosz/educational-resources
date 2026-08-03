#pragma once

#include <string>
#include <variant>

namespace rpn {

    struct Token {
        enum class TokenType {
            NUMBER,
            OPERATOR,
            END
        } type;

        std::variant<double, std::string> value;
    };

}
