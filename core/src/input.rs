//! Input state handling.

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct CnInputState {
    pub x: f32,
    pub y: f32,
    pub is_mouse_down: i32,
    pub is_mouse_clicked: i32,
}

impl CnInputState {
    pub fn new(x: f32, y: f32, down: bool, clicked: bool) -> Self {
        Self {
            x, y,
            is_mouse_down: if down { 1 } else { 0 },
            is_mouse_clicked: if clicked { 1 } else { 0 },
        }
    }

    pub fn mouse_down(&self) -> bool { self.is_mouse_down != 0 }
    pub fn mouse_clicked(&self) -> bool { self.is_mouse_clicked != 0 }
}