#pragma once
#include <map>
#include <memory>
#include <string>

#include <rpn/stack.hpp>

namespace rpn {
    class AbstractComputationStrategy {
    public:
        explicit AbstractComputationStrategy(std::string op) : operator_(std::move(op)) {}
        virtual ~AbstractComputationStrategy() = default;

        [[nodiscard]] const std::string& get_operator() const noexcept {
            return operator_;
        }
        virtual void call(IDoubleStack& stack) const = 0;
    private:
        std::string operator_;
    };

    class PlusStrategy : public AbstractComputationStrategy {
    public:
        PlusStrategy() : AbstractComputationStrategy("+") {}
        void call(IDoubleStack& stack) const noexcept override;
    };

    class MinusStrategy : public AbstractComputationStrategy {
    public:
        MinusStrategy() : AbstractComputationStrategy("-") {}
        void call(IDoubleStack& stack) const noexcept override;
    };

    class MultStrategy : public AbstractComputationStrategy {
    public:
        MultStrategy() : AbstractComputationStrategy("*") {}
        void call(IDoubleStack& stack) const noexcept override;
    };

    class DivStrategy : public AbstractComputationStrategy {
    public:
        DivStrategy() : AbstractComputationStrategy("/") {}
        void call(IDoubleStack& stack) const noexcept override;
    };

    class ComputationStrategySelector {
    public:
        using StartegyPtr = std::unique_ptr<AbstractComputationStrategy>;

        ComputationStrategySelector() = default;
        ComputationStrategySelector& register_strategy(StartegyPtr strategy);
        [[nodiscard]] AbstractComputationStrategy& get_strategy(const std::string& key) const;
    private:
        std::map<std::string, ComputationStrategySelector::StartegyPtr> startegies_;
    };

}
