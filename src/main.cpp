#include "main.h"
#include "util.h"

int main(int argc, char* argv[]) {
	std::string str = std::format("\"{}\"", collect_args(argc, argv));
	std::cout << str << "\n";
}


