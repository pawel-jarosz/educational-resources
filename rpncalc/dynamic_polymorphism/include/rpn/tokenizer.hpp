#pragma once

#include <sstream>

#include <rpn/token.hpp>

namespace rpn {

    class ITokenizer {
    public:
        virtual ~ITokenizer() = default;
        virtual void reset(std::string input) = 0;
        virtual Token next() = 0;
    };

    class Tokenizer : public ITokenizer {
    public:
        explicit Tokenizer(std::string input);
        Tokenizer() = default;
        void reset(std::string input) override;
        Token next() override;
    private:
        std::stringstream ss_;
    };

}
