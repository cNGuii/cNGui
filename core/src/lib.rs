use std::os::raw::{c_char, c_void};

#[repr(C)]
pub struct CnInitParams { pub size: u32, pub title: *const c_char, pub width: i32, pub height: i32 }
#[repr(C)]
pub struct CnDrawData { pub clear_color: u32 }
#[repr(C)]
pub struct CnInputState { pub x: f32, pub y: f32, pub is_mouse_down: i32, pub is_mouse_clicked: i32 }
#[repr(C)]
pub struct CnRect { pub x: f32, pub y: f32, pub w: f32, pub h: f32 }

pub struct CnContext { 
    pub draw_data: CnDrawData, 
    pub should_close: bool,
    pub input: CnInputState,
}

#[no_mangle]
pub extern "C" fn cn_init(_params: *const CnInitParams) -> *mut c_void {
    let ctx = Box::new(CnContext { 
        draw_data: CnDrawData { clear_color: 0x112233FF }, 
        should_close: false,
        input: CnInputState { x: 0.0, y: 0.0, is_mouse_down: 0, is_mouse_clicked: 0 },
    });
    Box::into_raw(ctx) as *mut c_void
}

#[no_mangle]
pub extern "C" fn cn_set_input(ctx: *mut c_void, input: *const CnInputState) {
    let ctx = unsafe { &mut *(ctx as *mut CnContext) };
    if !input.is_null() {
        // Copy the memory from C into our Rust struct
        ctx.input = unsafe { std::ptr::read(input) };
    }
}

#[no_mangle]
pub extern "C" fn cn_button(ctx: *mut c_void, bounds: CnRect, _label: *const c_char) -> i32 {
    let ctx = unsafe { &mut *(ctx as *mut CnContext) };
    let input = &ctx.input;

    // 1. Hit test: Is the mouse inside the rectangle?
    let hit = input.x >= bounds.x && input.x <= (bounds.x + bounds.w) &&
              input.y >= bounds.y && input.y <= (bounds.y + bounds.h);

    // 2. Return 1 (true) if it was hit AND clicked
    if hit && input.is_mouse_clicked == 1 {
        return 1; 
    }
    0
}

#[no_mangle]
pub extern "C" fn cn_begin_frame(_ctx: *mut c_void) {}

#[no_mangle]
pub extern "C" fn cn_end_frame(_ctx: *mut c_void) {}

#[no_mangle]
pub extern "C" fn cn_get_draw_data(ctx: *mut c_void) -> *const CnDrawData {
    let ctx = unsafe { &*(ctx as *mut CnContext) };
    &ctx.draw_data
}

#[no_mangle]
pub extern "C" fn cn_should_close(ctx: *const c_void) -> i32 {
    let ctx = unsafe { &*(ctx as *const CnContext) };
    if ctx.should_close { 1 } else { 0 }
}

#[no_mangle]
pub extern "C" fn cn_shutdown(ctx: *mut c_void) {
    if !ctx.is_null() { unsafe { drop(Box::from_raw(ctx as *mut CnContext)); } }
}
