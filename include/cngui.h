#ifndef CNGUI_H
#define CNGUI_H
#include <stdint.h>
#ifdef __cplusplus
extern "C" {
#endif

#define CNGUI_ABI_VERSION 1
typedef struct cn_context cn_context;

typedef struct {
    uint32_t size;
    const char* title;
    int width;
    int height;
} cn_init_params;

typedef struct {
    uint32_t clear_color;
} cn_draw_data;

// --- NEW IN M2: Input and Geometry ---

typedef struct {
    float x;
    float y;
    int is_mouse_down;
    int is_mouse_clicked;
} cn_input_state;

typedef struct {
    float x, y, w, h;
} cn_rect;

// Lifecycle & Frame
cn_context* cn_init(const cn_init_params* params);
void cn_shutdown(cn_context* ctx);
void cn_begin_frame(cn_context* ctx);
void cn_end_frame(cn_context* ctx);
int cn_should_close(const cn_context* ctx);
const cn_draw_data* cn_get_draw_data(const cn_context* ctx);

// NEW IN M2: Input & Widgets
void cn_set_input(cn_context* ctx, const cn_input_state* input);
int cn_button(cn_context* ctx, cn_rect bounds, const char* label);

#ifdef __cplusplus
}
#endif
#endif
