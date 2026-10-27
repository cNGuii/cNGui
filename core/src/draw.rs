//! Draw data structures.

use crate::math::Color32;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CnDrawData {
    pub clear_color: u32,
}

impl Default for CnDrawData {
    fn default() -> Self {
        Self { clear_color: Color32::new(17, 34, 51, 255).as_u32() }
    }
}

pub enum DrawCommand {
    Rect { x: f32, y: f32, w: f32, h: f32, color: Color32 },
    Text { x: f32, y: f32, text: String, color: Color32 },
}

pub struct DrawList {
    pub commands: Vec<DrawCommand>,
}

impl DrawList {
    pub fn new() -> Self { Self { commands: Vec::new() } }
    pub fn clear(&mut self) { self.commands.clear(); }
    
    pub fn push_rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: Color32) {
        self.commands.push(DrawCommand::Rect { x, y, w, h, color });
    }
}