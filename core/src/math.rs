//! Math types for geometry and colors.

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CnRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl CnRect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    pub fn contains(&self, px: f32, py: f32) -> bool {
        px >= self.x && px <= (self.x + self.w) &&
        py >= self.y && py <= (self.y + self.h)
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.x < other.x + other.w &&
        self.x + self.w > other.x &&
        self.y < other.y + other.h &&
        self.y + self.h > other.y
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Color32 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color32 {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    
    pub fn as_u32(&self) -> u32 {
        ((self.a as u32) << 24) |
        ((self.b as u32) << 16) |
        ((self.g as u32) << 8) |
        (self.r as u32)
    }
}