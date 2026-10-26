use std::os::raw::{c_char, c_void};

#[repr(C)]
pub struct CnInitParams {
    pub size: u32,
    pub title: *const c_char,
    pub width: i32,
    pub height: i32,
}

#[repr(C)]
pub struct CnDrawData {
    pub clear_color: u32,
}

// This struct lives in Rust memory, but C holds the pointer
pub struct CnContext {
    pub draw_data: CnDrawData,
    pub should_close: bool,
}

#[no_mangle]
pub extern "C" fn cn_init(_params: *const CnInitParams) -> *mut c_void {
    let ctx = Box::new(CnContext {
        draw_data: CnDrawData { 
            clear_color: 0x112233FF // A nice dark blue
        },
        should_close: false,
    });
    
    // Leak the box so the pointer stays valid for C
    Box::into_raw(ctx) as *mut c_void
}

#[no_mangle]
pub extern "C" fn cn_begin_frame(_ctx: *mut c_void) {
    // In the future, we clear the draw list here
}

#[no_mangle]
pub extern "C" fn cn_end_frame(_ctx: *mut c_void) {
    // In the future, we finalize the draw list here
}

#[no_mangle]
pub extern "C" fn cn_get_draw_data(ctx: *mut c_void) -> *const CnDrawData {
    // Safely cast the C pointer back to our Rust struct
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
    if !ctx.is_null() {
        // Reclaim the Box and drop it safely
        unsafe { drop(Box::from_raw(ctx as *mut CnContext)); }
    }
}
