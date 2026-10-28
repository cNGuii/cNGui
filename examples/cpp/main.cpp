#include "cngui.hpp"
#include <iostream>

int main() {
    std::cout << "Running C++ Example..." << std::endl;

    cngui::Context ctx(800, 600, "cNGui C++ Example");

    for (int frame = 0; frame < 3; frame++) {
        ctx.begin_frame();

        if (frame == 1) {
            ctx.set_input(60.0f, 60.0f, true, true);
        }

        if (ctx.button(50.0f, 50.0f, 100.0f, 40.0f, "Click Me")) {
            std::cout << "C++ Example: Button clicked!" << std::endl;
        }

        ctx.end_frame();
    }

    std::cout << "C++ Example shutdown cleanly." << std::endl;
    return 0;
}
