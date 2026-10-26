#ifndef CNGUI_H
#define CNGUI_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/* 
 * ABI Versioning
 * Bindings must check this at runtime to ensure they are linking 
 * against a compatible core.
 */
#define CNGUI_ABI_VERSION 1

/* 
 * Opaque Handles
 * The core state lives in Rust. C/C++ only ever see pointers to it.
 */
typedef struct cn_context cn_context;

/*
 * Initialization Parameters
 * We pass a struct with a 'size' field so we can add fields in the 
 * future without breaking older bindings.
 */
typedef struct {
    uint32_t size;          // Must be set to sizeof(cn_init_params)
    const char* title;      // Window title (borrowed UTF-8)
    int width;              // Window width in pixels
    int height;             // Window height in pixels
} cn_init_params;


/* Add this to cngui.h before the extern "C" closing brace */

typedef struct {
    uint32_t clear_color; // RGBA
} cn_draw_data;

/* Add this to your function declarations */
const cn_draw_data* cn_get_draw_data(const cn_context* ctx);

/* 
 * Lifecycle & ABI
 */
int cn_abi_version(void);
cn_context* cn_init(const cn_init_params* params);
void cn_shutdown(cn_context* ctx);

/*
 * Immediate Mode Frame Loop
 */
void cn_begin_frame(cn_context* ctx);
void cn_end_frame(cn_context* ctx);

/*
 * Window State
 */
int cn_should_close(const cn_context* ctx);

#ifdef __cplusplus
}
#endif

#endif /* CNGUI_H */
