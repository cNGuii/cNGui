#include "cngui.h"
#include <stdio.h>

int main(void) {
    cn_init_params params = { sizeof(cn_init_params), "cNGui Test", 800, 600 };
    
    // 1. Init
    cn_context* ctx = cn_init(&params);
    if (!ctx) return 1;

    // 2. Frame Loop
    int frames = 0;
    while (!cn_should_close(ctx) && frames < 1) {
        cn_begin_frame(ctx);
        
        // (Widgets will go here in M2)
        
        cn_end_frame(ctx);

        // 3. Consume Draw Data
        const cn_draw_data* draw_data = cn_get_draw_data(ctx);
        printf("Frame %d: Clear Color is 0x%08X\n", frames, draw_data->clear_color);
        
        frames++;
    }

    // 4. Shutdown
    cn_shutdown(ctx);
    printf("Clean shutdown.\n");
    return 0;
}
