#![cfg(target_arch = "wasm32")]

extern crate tailwind_merge_rs;

use tailwind_merge_rs::tw_merge;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_merges_classes_with_per_side_border_colors_correctly() {
    assert_eq!(
        tw_merge("border-t-some-blue border-t-other-blue"),
        "border-t-other-blue"
    );
    assert_eq!(
        tw_merge("border-t-some-blue border-some-blue"),
        "border-some-blue"
    );
}
