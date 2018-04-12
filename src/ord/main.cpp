/**
 * Print the numerical values of the characters from stdin.
 *
 * 2015  Bence Kalmar
 */

#include <iostream>

int main()
{
    char c;
    while (std::cin.get(c)) {
        std::cout << c + 0 << ' ';
    }
    std::cout << '\n';

    return 0;
}
