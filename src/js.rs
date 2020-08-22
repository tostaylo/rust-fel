use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// JavaScript [console.log](https://developer.mozilla.org/en-US/docs/Web/API/Console/log)
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}
