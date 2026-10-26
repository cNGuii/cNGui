#include "cngui.h"
#include <stdio.h>

int main(void) {
    cn_init_params params = { sizeof(cn_init_params), "cNGui Test", 800, 600 };
    cn_context* ctx = cn_init(&params);

    // Let's create a button at X=50, Y=50, Width=100, Height=40
    cn_rect button_bounds = { 50.0f, 50.0f, 100.0f, 40.0f };

    for (int frames = 0; frames < 3; frames++) {
        cn_begin_frame(ctx);

        // Mock Input
        cn_input_state input = { 0 };
        if (frames == 1) {
            printf("[Frame 1] Simulating click at (60, 60) - INSIDE button.\n");
            input.x = 60.0f; input.y = 60.0f;
            input.is_mouse_clicked = 1;
        } else if (frames == 2) {
            printf("[Frame 2] Simulating click at (10, 10) - OUTSIDE button.\n");
            input.x = 10.0f; input.y = 10.0f;
            input.is_mouse_clicked = 1;
        }
        
        // Send input to Rust
        cn_set_input(ctx, &input);

        // Test the button!
        if (cn_button(ctx, button_bounds, "Click Me")) {
            printf("  -> RESULT: Button returned TRUE (Clicked!)\n");
        } else {
            printf("  -> RESULT: Button returned FALSE\n");
        }

        cn_end_frame(ctx);
    }

    cn_shutdown(ctx);
    return 0;
}
