#include <iostream>

#include <rpn/tokenizer.hpp>
#include <rpn/stack.hpp>
#include <rpn/computation_strategy.hpp>
#include <rpn/calculator.hpp>

void build_strategy_selector(rpn::ComputationStrategySelector& selector) {
    selector.build<rpn::PlusStrategy,
    rpn::MinusStrategy, 
    rpn::MultStrategy, 
    rpn::DivStrategy>();
}

int main(int argc, char* argv[]) {
    std::cout << "Please provide expression after command prompt '>' or type \"exit\"\n";

    rpn::DoubleStackStdWrapper rpn_stack;
    rpn::Tokenizer tokenizer;
    rpn::ComputationStrategySelector selector;
    selector.build<rpn::PlusStrategy,
                   rpn::MinusStrategy, 
                   rpn::MultStrategy, 
                   rpn::DivStrategy>();

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
