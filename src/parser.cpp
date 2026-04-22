#include "main.h"
#include "parser.h"

std::string collect_args(int argc, char* argv[]) {
	std::string str = "";
	for (int i=1;i<argc;i++) {
		str += argv[i];
		if (i == argc-1) continue;
		str += " ";
	}

	return str;
}

std::vector<Token> tokenize(std::string input) {
	return std::vector<Token>{};
}
