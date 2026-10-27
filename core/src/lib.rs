//! cNGui Core Engine
//! A polyglot, immediate-mode GUI library.

pub mod context;
pub mod draw;
pub mod input;
pub mod math;
pub mod widgets;

use std::os::raw::{c_char, c_void};

pub use context::CnContext;
pub use draw::CnDrawData;
pub use input::CnInputState;
pub use math::CnRect;

#[repr(C)]
pub struct CnInitParams {
    pub size: u32,
    pub title: *const c_char,
    pub width: i32,
    pub height: i32,
}

#[no_mangle]
pub extern "C" fn cn_init(params: *const CnInitParams) -> *mut c_void {
    let ctx = context::init_context(params);
    Box::into_raw(Box::new(ctx)) as *mut c_void
}

#[no_mangle]
pub extern "C" fn cn_shutdown(ctx: *mut c_void) {
    if !ctx.is_null() {
        unsafe { drop(Box::from_raw(ctx as *mut CnContext)); }
    }
}

#[no_mangle]
pub extern "C" fn cn_begin_frame(ctx: *mut c_void) {
    let ctx = unsafe { &mut *(ctx as *mut CnContext) };
    context::begin_frame(ctx);
}

#[no_mangle]
pub extern "C" fn cn_end_frame(ctx: *mut c_void) {
    let ctx = unsafe { &mut *(ctx as *mut CnContext) };
    context::end_frame(ctx);
}

#[no_mangle]
pub extern "C" fn cn_set_input(ctx: *mut c_void, input: *const CnInputState) {
    let ctx = unsafe { &mut *(ctx as *mut CnContext) };
    if !input.is_null() {
        ctx.input = unsafe { std::ptr::read(input) };
    }
}

#[no_mangle]
pub extern "C" fn cn_button(ctx: *mut c_void, bounds: CnRect, _label: *const c_char) -> i32 {
    let ctx = unsafe { &mut *(ctx as *mut CnContext) };
    if widgets::button(ctx, bounds) { 1 } else { 0 }
}

#[no_mangle]
pub extern "C" fn cn_get_draw_data(ctx: *mut c_void) -> *const CnDrawData {
    let ctx = unsafe { &*(ctx as *mut CnContext) };
    &ctx.draw_data
}