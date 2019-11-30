use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlElement};

use crate::context::Context;
use crate::java_class::default::setup_class_map;
use crate::java_class::custom::*;
use crate::string_pool::StringPool;
use std::path::Path;

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

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn run_wasm(class_name: &str, inputs: &[u8]) {
    let mut string_pool = StringPool::new();

    let (class_file, _pc_count) = Custom::new(&mut string_pool, inputs, 0);
    let class_map = setup_class_map(&mut string_pool);
    let parent_path = if let Some(parent_path) = Path::new(&class_name).parent() {
        parent_path.to_str().unwrap()
    } else {
        "./"
    };

    let mut context = Context::new(&mut string_pool, class_map, &class_file, parent_path);
    context.run_entry_file(&mut string_pool, class_file);
}

#[cfg(unix)]
pub fn print_log(value: &str) {
    println!("{}", value);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn print_log(value: &str) {
    alert(value);
}
