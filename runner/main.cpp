#include <fmt/core.h>

extern "C" {
    int get_sum(int x, int y);
    void get_http();
}

extern "C" int app_main(int argc, char *argv[])
{
    fmt::print("Hello {}!\n", "new world");

    int x = 15;
    int y = 27;
    fmt::print("In rust, {} + {} = {}\n", x, y, get_sum(x, y));

    fmt::print("BEFORE GET\n");
    get_http();
    fmt::print("AFTER GET\n");

    return 0;
}