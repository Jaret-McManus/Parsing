#include "main.h"
#include "token.h"

std::vector<Token> tokenize(std::string input) {
	std::vector<Token> tokens{};

	int32_t length = input.length();
	for (int32_t i=0; i<length; i++) {
		tokens.push_back(Token(input.substr(i,1)));
	}
	return tokens;
}