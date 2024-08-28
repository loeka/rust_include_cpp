#include "cpplib.h"

#include <iostream>
#include <cstring>

int add_one(int a) {
    return a+1;
}

void hello_cout() {
    std::cout << "Hello World!" << std::endl;
}

const char* hello_from_c() {
    return "Hello";
}

const char* reply(const char * str) {
    std::cout << "C++ Code gets: " << str << std::endl;
    std::string tmp = std::string("Your input was: '") + str;
    std::cout << "Debug from C++: " << tmp << std::endl;

    // Allocate the string on the heap -_-
    char* result = new char[tmp.size() + 1];
    std::strcpy(result, tmp.c_str());

    return result;
}

void free_string_reply(const char* str) {
    delete[] str;
}

const char* reply2(const char* str) {
    static char buffer[128];
    std::string tmp = std::string("Your input was: '") + str;

    std::strncpy(buffer, tmp.c_str(), sizeof(buffer) - 1);
    buffer[sizeof(buffer) - 1] = '\0';
    return buffer;
}

void reply3(char* buffer, unsigned int len) {
    std::string str = "Hello World!";
    auto lenStr = str.length();

    if (len > lenStr) {
        std::strncpy(buffer, str.c_str(), lenStr);
        buffer[lenStr] = '\0';
    }
}
