#include <iostream>
#include <sstream>
#include <stack>

void execute_operation(std::stack<double>& stack, const std::string& op) {
    double a = stack.top();
    stack.pop();
    double b = stack.top();
    stack.pop();
    if (op == "+") {
        stack.push(a + b);
    }
    else if (op == "-") {
        stack.push(b - a);
    }
    else if (op == "*") {
        stack.push(a * b);
    }
    else if (op == "/") {
        stack.push(b / a);
    }
}

double compute_rpn(const std::string& input) {
    std::stringstream ss{input};
    std::string element;
    std::stack<double> rpn_stack;
    while (!ss.eof()) {
        ss >> element;
        if (element == "+" or element == "-" or element == "*"
            or element == "*" or element == "/") {
            execute_operation(rpn_stack, element);
        }
        else {
            rpn_stack.push(std::stod(element));
        }
    }
    return rpn_stack.top();
}

int main(int argc, char* argv[]) {
    std::cout << "Please provide expression after command prompt or type \"exit\"\n";
    std::string input;

    bool shall_continue = true;
    while (shall_continue) {
        std::cout << "? ";
        std::getline(std::cin, input);
        if (input == "exit" ) {
            shall_continue = false;
        }
        else {
            std::cout << "=> " << compute_rpn(input) << "\n";
        }
    }
}
