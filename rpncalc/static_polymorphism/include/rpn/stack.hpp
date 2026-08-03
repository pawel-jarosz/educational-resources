#pragma once

namespace rpn {

    template<template<typename T> class StackType, typename StackedValue>
    concept IsStack = requires(StackType<StackedValue> stack) {
        { stack.push(1.0) } -> std::same_as<void>;
        { stack.pop() } -> std::same_as<void>;
        { stack.top() } -> std::same_as<StackedValue>;
    };
}
