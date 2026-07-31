#pragma once
#include "main.h"

enum class TokenType {
	NUMBER,
	ADD,
	LPAREN,
	RPAREN
};

class Token {
    public:
        TokenType type;
        double value;

        Token(TokenType p_type)
        	: type(p_type), value(0.0) {}

        Token(TokenType p_type, double p_value)
        	: type(p_type), value(p_value) {}

        std::string to_string() {
        	switch(type) {
        		case TokenType::NUMBER:
        			return std::format("{}", value);
        		case TokenType::ADD:
        			return "+";
        		case TokenType::LPAREN:
        			return "(";
        		case TokenType::RPAREN:
        			return ")";
        		default: throw std::runtime_error("Invalid Token!");
        	}
        }
};