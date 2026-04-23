#include "main.h"

enum class TokenType {
	NUMBER,
	ADD,
	LPAREN,
	RPAREN
};

class Token {
    public:
        std::string text;
        Token(std::string p_text) {
            text = p_text;
        }
};