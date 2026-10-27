//! Immediate-mode widgets.

use crate::context::CnContext;
use crate::math::{CnRect, Color32};

pub fn button(ctx: &mut CnContext, bounds: CnRect) -> bool {
    let is_hovered = bounds.contains(ctx.input.x, ctx.input.y);
    
    // Draw the button
    let color = if is_hovered && ctx.input.mouse_down() {
        Color32::new(100, 100, 100, 255) // Pressed
    } else if is_hovered {
        Color32::new(150, 150, 150, 255) // Hovered
    } else {
        Color32::new(200, 200, 200, 255) // Normal
    };

    ctx.draw_list.push_rect(bounds.x, bounds.y, bounds.w, bounds.h, color);

    // Return true if clicked
    is_hovered && ctx.input.mouse_clicked()
}

pub fn checkbox(ctx: &mut CnContext, bounds: CnRect, value: &mut bool) -> bool {
    let is_hovered = bounds.contains(ctx.input.x, ctx.input.y);
    let clicked = is_hovered && ctx.input.mouse_clicked();

    if clicked {
        *value = !*value;
    }

    let color = if *value {
        Color32::new(0, 255, 0, 255) // Checked
    } else {
        Color32::new(200, 200, 200, 255) // Unchecked
    };

    ctx.draw_list.push_rect(bounds.x, bounds.y, bounds.w, bounds.h, color);
    clicked
}