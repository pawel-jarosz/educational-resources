//
// Created by Paweł Jarosz on 4.07.2026.
//

#include <rpn/stack.hpp>

namespace rpn {

bool DoubleStackStdWrapper::empty() {
    return stack_.empty();
}

void DoubleStackStdWrapper::push(double value) {
    stack_.push(value);
}

void DoubleStackStdWrapper::pop() {
    stack_.pop();
}

double DoubleStackStdWrapper::top() const {
    return stack_.top();
}

}
