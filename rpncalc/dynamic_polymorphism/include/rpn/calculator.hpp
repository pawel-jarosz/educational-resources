#pragma once

#include <rpn/computation_strategy.hpp>
#include <rpn/tokenizer.hpp>
#include <rpn/stack.hpp>

namespace rpn {

class Calculator {
public:
    Calculator(ComputationStrategySelector& selector, IDoubleStack& double_stack)
        : selector_(selector), double_stack_(double_stack) {}
    double compute(ITokenizer& tokenizer);
private:
    ComputationStrategySelector& selector_;
    IDoubleStack& double_stack_;
};

}
