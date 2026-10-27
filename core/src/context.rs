//! The central context holding all state.

use crate::draw::{CnDrawData, DrawList};
use crate::input::CnInputState;
use crate::CnInitParams;

pub struct CnContext {
    pub draw_data: CnDrawData,
    pub draw_list: DrawList,
    pub input: CnInputState,
    pub should_close: bool,
    pub width: i32,
    pub height: i32,
}

pub fn init_context(params: *const CnInitParams) -> CnContext {
    let mut width = 800;
    let mut height = 600;

    if !params.is_null() {
        let p = unsafe { &*params };
        width = p.width;
        height = p.height;
    }

    CnContext {
        draw_data: CnDrawData::default(),
        draw_list: DrawList::new(),
        input: CnInputState::default(),
        should_close: false,
        width,
        height,
    }
}

pub fn begin_frame(ctx: &mut CnContext) {
    ctx.draw_list.clear();
}

pub fn end_frame(_ctx: &mut CnContext) {
    // Finalize draw list here
}