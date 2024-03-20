pub fn tw_merge(classes: &str) -> String {
    inner::tw_merge(classes).unwrap().as_string().unwrap()
}

mod inner {
    use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

    #[wasm_bindgen(module = "/src/tailwind-merge.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn tw_merge(classes: &str) -> Result<JsValue, JsValue>;
    }
}

