#include <iostream>

#include <rpn/tokenizer.hpp>
#include <rpn/stack.hpp>
#include <rpn/computation_strategy.hpp>
#include <rpn/calculator.hpp>

void build_strategy_selector(rpn::ComputationStrategySelector& selector) {
    selector
    .register_strategy(std::make_unique<rpn::PlusStrategy>())
    .register_strategy(std::make_unique<rpn::MinusStrategy>())
    .register_strategy(std::make_unique<rpn::MultStrategy>())
    .register_strategy(std::make_unique<rpn::DivStrategy>());
}

int main(int argc, char* argv[]) {
    std::cout << "Please provide expression after command prompt '>' or type \"exit\"\n";

    rpn::DoubleStackStdWrapper rpn_stack;
    rpn::Tokenizer tokenizer;
    rpn::ComputationStrategySelector selector;
    build_strategy_selector(selector);

    rpn::Calculator calculator{selector, rpn_stack};

    std::string input;

    bool shall_continue = true;
    while (shall_continue) {
        std::cout << "? ";
        std::getline(std::cin, input);
        tokenizer.reset(input);
        if (input == "exit" ) {
            shall_continue = false;
        }
        else {
            std::cout << "=> " << calculator.compute(tokenizer) << "\n";
        }
    }
}
