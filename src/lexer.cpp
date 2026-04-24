#include "main.h"
#include "token.h"
#include "lexer.h"

std::vector<Token> Lexer::tokenize(std::string& input) {
	std::vector<Token> tokens{};

	size_t length = input.length();
	size_t position = 0;
	while (position < length) {
		const char ch = input.at(position);
		if(std::isspace(ch)) { 						//whitespace: ignore
			position++;
			continue;
		} else if(std::isdigit(ch) || ch == '.') { 	// number: consume number
			Token number = Lexer::consume_number(input, position);
			tokens.push_back(number);
			continue;
		}

		// other token type
		TokenType type;
		switch (ch) {
			case '+':
				type = TokenType::ADD; break;
            case '(':
            	type = TokenType::LPAREN; break;
            case ')':
            	type = TokenType::RPAREN; break;
            default:
            	throw std::runtime_error("Unexpected character");
		}
		Token token = Token(type);
		tokens.push_back(token);

		position++;
	}
	return tokens;
}

Token Lexer::consume_number(std::string& input, size_t& position) {
	std::string number_string{};
	while(true) {
		if (position >= input.length()) break;
		const char ch = input.at(position);
		if (std::isdigit(ch) || ch == '.') {
			number_string.push_back(ch);
			position++;
		} else {
			break;
		}
	}

	size_t pos;
	double value = std::stod(number_string, &pos);
	if (pos != number_string.length()) throw std::runtime_error("Invalid Number!");
	return Token(TokenType::NUMBER, value);
}