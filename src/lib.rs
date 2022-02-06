use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern alert {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
