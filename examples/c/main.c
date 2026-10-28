#include "cngui.h"
#include <stdio.h>

int main(void) {
    cn_init_params params = { sizeof(cn_init_params), "cNGui C Example", 800, 600 };
    cn_context* ctx = cn_init(&params);

    printf("Running C Example...\n");

    for (int frame = 0; frame < 3; frame++) {
        cn_begin_frame(ctx);

        cn_input_state input = { 0 };
        if (frame == 1) {
            input.x = 60.0f; input.y = 60.0f;
            input.is_mouse_clicked = 1;
        }
        cn_set_input(ctx, &input);

        cn_rect btn_bounds = { 50.0f, 50.0f, 100.0f, 40.0f };
        if (cn_button(ctx, btn_bounds, "Click Me")) {
            printf("C Example: Button clicked!\n");
        }

        cn_end_frame(ctx);
    }

    cn_shutdown(ctx);
    printf("C Example shutdown cleanly.\n");
    return 0;
}
