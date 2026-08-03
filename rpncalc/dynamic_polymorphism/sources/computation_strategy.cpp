#include <rpn/computation_strategy.hpp>

namespace rpn {

void PlusStrategy::call(IDoubleStack& stack) const noexcept {
    auto a = stack.top();
    stack.pop();
    auto b = stack.top();
    stack.pop();
    stack.push(b + a);
}

void MinusStrategy::call(IDoubleStack& stack) const noexcept {
    auto a = stack.top();
    stack.pop();
    auto b = stack.top();
    stack.pop();
    stack.push(b - a);
}

void MultStrategy::call(IDoubleStack& stack) const noexcept {
    auto a = stack.top();
    stack.pop();
    auto b = stack.top();
    stack.pop();
    stack.push(b * a);
}

void DivStrategy::call(IDoubleStack& stack) const noexcept {
    auto a = stack.top();
    stack.pop();
    auto b = stack.top();
    stack.pop();
    stack.push(b / a);
}

AbstractComputationStrategy& ComputationStrategySelector::get_strategy(const std::string& key) const {
    return *startegies_.at(key);
}

ComputationStrategySelector& ComputationStrategySelector::register_strategy(StartegyPtr strategy) {
    startegies_.emplace(strategy->get_operator(), std::move(strategy));
    return *this;
}

}
