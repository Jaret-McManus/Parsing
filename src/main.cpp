#include <iostream>
#include <format>
#include "main.h"

int main(int argc, char* argv[]) {
	std::string str = std::format("\"{}\"", collect_args(argc, argv));
	std::cout << str << "\n";
}

std::string collect_args(int argc, char* argv[]) {
	std::string str = "";
	for (int i=1;i<argc;i++) {
		str += argv[i];
		if (i == argc-1) continue;
		str += " ";
	}

	return str;

}
