#![cfg(target_arch = "wasm32")]

extern crate tailwind_merge_rs;

use tailwind_merge_rs::tw_merge;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_merges_content_utilities_correctly() {
    assert_eq!(
        tw_merge("content-['hello'] content-[attr(data-content)]"
        ),
        "content-[attr(data-content)]"
    );
}
