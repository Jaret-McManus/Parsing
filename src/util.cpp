#include "main.h"

std::string collect_args(int argc, char* argv[]) {
	std::string str = "";
	for (int32_t i=1;i<argc;i++) {
		str += argv[i];
		if (i == argc-1) continue;
		str += " ";
	}

	return str;
}