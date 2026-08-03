#pragma once

#include <rpn/computation_strategy.hpp>

#include <gmock/gmock.h>

namespace rpn::tests::mocks {

    class StrategyMock : public AbstractComputationStrategy {
        public:
        StrategyMock(std::string op) : AbstractComputationStrategy(std::move(op)) {}
        MOCK_METHOD(void, call, (IDoubleStack&), (const override));
    };

}
