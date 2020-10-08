mod utils;


use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    // let canvas=documen
    // Manufacture the element we're gonna append
    let val = document.create_element("canvas")?;
    let canvas = document.get_element_by_id("canvas").unwrap();
    // val.set_inner_html("Hello from Rust!");

    // body.append_child(&val)?;
    let canvas: web_sys::HtmlCanvasElement = canvas
    .dyn_into::<web_sys::HtmlCanvasElement>()
    .map_err(|_| ())
    .unwrap();

    let windows: web_sys::Window =web_sys::window().unwrap();


    let context = canvas
    .get_context("2d")
    .unwrap()
    .unwrap()
    .dyn_into::<web_sys::CanvasRenderingContext2d>()
    .unwrap();
    
    // context.width=windows.innerWidth;
    // context.set_height(windows.innerHeight);
//context.set_fill_style(&"#0000FF".into());     
    context.set_fill_style(&"rgb(150,50,0)".into());        

    context.fill_rect(15.0, 15.0, 80.0, 80.0);

    // web_sys::console::log_2(&"Color : %s ".into(),&context.fill_style().into());
    Ok(())
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, movesquare!");
}
