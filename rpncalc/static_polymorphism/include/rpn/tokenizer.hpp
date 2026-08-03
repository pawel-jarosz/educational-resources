#pragma once

#include <sstream>

#include <rpn/token.hpp>

namespace rpn {

    template<template<typename T> class TokenizerType, typename T>
    concept IsTokenizer = requires(TokenizerType<T> tokenizer, std::string input) {
        { tokenizer.reset(input) } -> std::same_as<void>;
        { tokenizer.next() } -> std::same_as<Token<T>>;
    };

    class RegularArithmeticTokenizer {
    public:
        explicit RegularArithmeticTokenizer(std::string input);
        RegularArithmeticTokenizer() = default;
        void reset(std::string input);
        Token<double> next();
    private:
        std::stringstream ss_;
    };

}
