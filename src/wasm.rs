use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlElement};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_u8_array(a: &[u8]);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_u16(a: u16);
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("should have a window in this context");
    let document = window.document().expect("window should have a document");
    setup_clicker(&document);

    Ok(())
}

pub fn setup_clicker(document: &Document) {
    let mut clicks = 0;
    let a = Closure::wrap(Box::new(move || {
        clicks += 1;
        web_sys::console::log_1(&format!("{}", clicks).into());
        // alert(&format!("{}", clicks));
    }) as Box<dyn FnMut()>);

    let _ = document
        .query_selector("#emitButton")
        .expect("should have #button on the page")
        .unwrap()
        .dyn_ref::<HtmlElement>()
        .expect("#emitButton be an `HtmlElement`")
        .set_onclick(Some(a.as_ref().unchecked_ref()));

    a.forget();
}

#[cfg(unix)]
pub fn greet(name: &str) {
    println!("Hello, {}! in standard environment", name);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}! in wasm", name));
}
