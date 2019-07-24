use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use js_sys::Math;
use std::f64::consts::PI;
use crate::utils::{
    document,
    canvas,
    request_animation_frame,
    get_context,
    width,
    height
};
use web_sys::console;

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

    // let f = Rc::new(context());
    let f = Rc::new(RefCell::new(None));

    let g = f.clone();
    let mut i = 0;
    let canvas = canvas();
    let attr = document().create_attribute("id")?;
    attr.set_value("animation");
    canvas.set_attribute_node(&attr)?;

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        if i > 300 {
			// body().set_text_content(Some("All done!"));
             console::log_1(&JsValue::from_str("all done.."));

            // Drop our handle to this closure so that it will get cleaned
            // up once we return.
            let _ = f.borrow_mut().take();
            return;
        }
        draw_circles();
        request_animation_frame(f.borrow().as_ref().unwrap());

        i += 1;
    }) as Box<dyn FnMut()> ));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}

fn draw_circles() {
    let context = get_context("animation");
    context.clear_rect(0.0, 0.0, width() as f64, height() as f64);

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

    for mut circle in circles {
        context.begin_path();
        context.set_fill_style(&JsValue::from_serde(&circle.color).unwrap());
        context.arc(circle.x, circle.y, circle.radius, 0.0, PI * 2.0).expect("something wrong");
        context.fill();
        context.close_path();

        if circle.x < 0.0 || circle.x > width() as f64 { circle.dx=-circle.dx }
        if circle.y < 0.0 || circle.y > height() as f64 { circle.dy=-circle.dy }
        circle.x += circle.dx;
        circle.y += circle.dy;
    }
}
