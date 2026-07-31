#include "main.h"
#include "token.h"

class Lexer {
public:
	static std::vector<Token> tokenize(std::string& input);
	static Token consume_number(std::string& input, size_t& position);
};