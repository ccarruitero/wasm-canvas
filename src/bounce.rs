use wasm_bindgen::prelude::*;
// use std::rc::Rc;
use js_sys::Math;
use std::f64::consts::PI;
use crate::utils::{
    window,
    request_animation_frame,
    document, canvas,
    context,
    width,
    height
};

extern crate serde_derive;

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
    color: String,
    dx: f64,
    dy: f64
}

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {

    // draw circles
    draw_circles();

    Ok(())
}

fn draw_circles() {
    // ensure not creating new canvas
    context().clear_rect(0.0, 0.0, width() as f64, height() as f64);

    let mut circles = vec![];

    let circle = Circle {
        radius: 90.3,
        color: "rgba(229,173,124,0.7)".to_string(),
        x: 995.0,
        y: 175.0,
        dx: Math::cos(PI * (Math::fround( Math::random()) as f64)),
        dy: Math::cos(PI * (Math::fround( Math::random()) as f64))
    };

    circles.push(circle);

    for circle in circles {
        context().begin_path();
        context().set_fill_style(&JsValue::from_serde(&circle.color).unwrap());
        context().arc(circle.x, circle.y, circle.radius, 0.0, PI * 2.0)?;
        context().fill();
        context().close_path();
    }
}
