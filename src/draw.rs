use wasm_bindgen::prelude::*;
use std::rc::Rc;
use wasm_bindgen::JsCast;
// use web_sys::console;

extern crate serde_derive;

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    document.body().unwrap().append_child(&canvas)?;
    let width = window.inner_width().unwrap().into_serde().unwrap();
    let height = window.inner_height().unwrap().into_serde().unwrap();
    canvas.set_width(width);
    canvas.set_height(height);

    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
    let context = Rc::new(context);
    {
        let context = context.clone();
        // let closure = Closure::wrap() as Box<>);
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            context.line_to(event.offset_x() as f64, event.offset_y() as f64);
            context.stroke();
            context.begin_path();
            context.move_to(event.offset_x() as f64, event.offset_y() as f64);
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    // console::log_1(&JsValue::from_str("drawing canvas from rust!"));
    Ok(())
}
