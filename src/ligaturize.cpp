#include <tclap/CmdLine.h>

#include <iostream>
#include <unordered_map>

const std::unordered_map<std::string, std::string> ligatures {
    {"IJ", "Ĳ"},
    {"OE", "Œ"},
    {"ff", "ﬀ"},
    {"ffi", "ﬃ"},
    {"ffl", "ﬄ"},
    {"fi", "ﬁ"},
    {"fl", "ﬂ"},
    {"ij", "ĳ"},
    {"oe", "œ"},
    {"st", "ﬆ"},
    {"ſt", "ﬅ"}
};

void ligaturize(std::string &s)
{
    for (auto p : ligatures) {
        decltype(s.size()) i = 0;
        while ((i = s.find(p.first, i)) != s.npos) {
            s.replace(i, p.first.size(), p.second);
        }
    }
}

int action()
{
    std::string line;
    while (std::getline(std::cin, line)) {
        ligaturize(line);
        std::cout << line << '\n';
    }
    return 0;
}

int main(int argc, char **argv)
{
    try {
        TCLAP::CmdLine parser(
            "Replace certain latin letter combinations with their corresponding"
            " ligatures from stdin, and print to stdout."
        );

        parser.parse(argc, argv);

        return action();
    } catch (const TCLAP::ArgException &e) {
        std::cerr << e.error() << '\n';
        std::exit(2);
    }
}
