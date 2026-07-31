#include "main.h"
#include "util.h"
#include "lexer.h"

int main(int argc, char* argv[]) {
	std::string str = collect_args( argc, argv );
	std::vector<Token> tokens = Lexer::tokenize( str );
	for (Token& token: tokens) {
		std::cout << token.to_string() << " ";
	}
	std::cout << std::endl;
}


