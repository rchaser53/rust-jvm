#[cfg(unix)]
use crate::utils::read_file;

#[allow(unused_imports)]
use wasm_bindgen::prelude::*;

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

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(module = "/web/map.js")]
extern "C" {
    pub fn get_file_content_from_js(key: &str) -> Vec<u8>;
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn get_file_content(key: &str) -> Vec<u8> {
    get_file_content_from_js(key)
}

#[cfg(unix)]
pub fn get_file_content(key: &str) -> Vec<u8> {
  read_file(&key).expect(&format!(
      "need to add handler for the case failed to find the class file: {}",
      &key
  ))
}

#[cfg(unix)]
pub fn print_log(value: &str) {
    println!("{}", value);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn print_log(value: &str) {
    log(value);
}
