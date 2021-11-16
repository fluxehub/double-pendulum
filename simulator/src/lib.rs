use std::f64;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn test(ctx: web_sys::CanvasRenderingContext2d) {
    ctx.begin_path();

    // Draw the outer circle.
    ctx.arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0).unwrap();

    // Draw the mouth.
    ctx.move_to(110.0, 75.0);
    ctx.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

    // Draw the left eye.
    ctx.move_to(65.0, 65.0);
    ctx.arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0).unwrap();

    // Draw the right eye.
    ctx.move_to(95.0, 65.0);
    ctx.arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0).unwrap();

    ctx.stroke();
}