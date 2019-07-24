use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// https://rustwasm.github.io/wasm-bindgen/examples/request-animation-frame.html

pub fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

pub fn width() -> f32 {
    window().inner_width().unwrap().into_serde().expect("no width")
}

pub fn height()  -> f32 {
    window().inner_height().unwrap().into_serde().expect("no height")
}

pub fn canvas() -> web_sys::HtmlCanvasElement {
    let canvas_el = document()
        .create_element("canvas").expect("no element")
        .dyn_into::<web_sys::HtmlCanvasElement>().expect("no HtmlCanvasElement");
    document().body().unwrap().append_child(&canvas_el).expect("couldnt append canvas");
    canvas_el.set_width(width() as u32);
    canvas_el.set_height(height() as u32);
    // canvas.style().set_property("border", "solid")?;
    canvas_el
} 

pub fn context() -> web_sys::CanvasRenderingContext2d {
    canvas()
        .get_context("2d").expect("no canvas")
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>().expect("no context")
}

pub fn get_context(canvas_id: &str) -> web_sys::CanvasRenderingContext2d {
    let canvas = document().get_element_by_id(canvas_id).expect("no canvas")
        .dyn_into::<web_sys::HtmlCanvasElement>().expect("no HtmlCanvasElement");
    canvas.get_context("2d").expect("no canvas")
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>().expect("no context")
}
