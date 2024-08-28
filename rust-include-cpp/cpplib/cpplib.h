#include <cstddef>
#include <cstdint>

extern "C" {
    int add_one(int a);
    void hello_cout();
    const char* hello_from_c();
    const char* reply(const char* str);
    void free_string_reply(const char* str);
    const char* reply2(const char* str);
    void reply3(char* buffer, unsigned int len);
}
