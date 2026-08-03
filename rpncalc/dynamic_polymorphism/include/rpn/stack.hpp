//
// Created by Paweł Jarosz on 4.07.2026.
//

#pragma once
#include <stack>

namespace rpn {

    class IDoubleStack {
    public:
        ~IDoubleStack() = default;
        virtual double top() const = 0;
        virtual void push(double value) = 0;
        virtual void pop() = 0;
        virtual bool empty() = 0;
    };

    class DoubleStackStdWrapper : public IDoubleStack {
    public:
        DoubleStackStdWrapper() = default;
        virtual double top() const override;
        virtual void push(double value) override;
        virtual void pop() override;
        virtual bool empty() override;
    protected:
        std::stack<double> stack_;
    };

}
