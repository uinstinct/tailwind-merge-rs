#![cfg(target_arch = "wasm32")]

extern crate tailwind_merge_rs;

use tailwind_merge_rs::tw_merge;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_does_not_alter_non_tailwind_classes() {
    assert_eq!(
        tw_merge(String::from("non-tailwind-class inline block")),
        "non-tailwind-class block"
    );
    assert_eq!(
        tw_merge(String::from("inline block inline-1")),
        "block inline-1"
    );
    assert_eq!(
        tw_merge(String::from("inline block i-inline")),
        "block i-inline"
    );
    assert_eq!(
        tw_merge(String::from("focus:inline focus:block focus:inline-1")),
        "focus:block focus:inline-1"
    );
}
