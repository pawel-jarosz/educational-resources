#include <rpn/tokenizer.hpp>

#include <string>
#include <sstream>

#include <boost/lexical_cast.hpp>

namespace rpn {

RegularArithmeticTokenizer::RegularArithmeticTokenizer(std::string input)
    : ss_(std::move(input)){

}

void RegularArithmeticTokenizer::reset(std::string input){
    std::stringstream new_input{std::move(input)};
    ss_.swap(new_input);
}

Token<double> RegularArithmeticTokenizer::next() {
    std::string element;
    ss_ >> element;
    try {
        double result = boost::lexical_cast<double>(element);
        return Token<double>(result);
    }
    catch (boost::bad_lexical_cast& err) {
        if (not element.empty()) {
            return Token<double>(element);
        }
    }
    return Token<double>();
}


}
