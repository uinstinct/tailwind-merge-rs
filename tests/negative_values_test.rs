#![cfg(target_arch = "wasm32")]

extern crate tailwind_merge;

use tailwind_merge::tw_merge;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_handles_negative_value_conflicts_correctly() {
    assert_eq!(tw_merge("-m-2 -m-5"), "-m-5");
    assert_eq!(tw_merge("-top-12 -top-2000"), "-top-2000");
}

#[wasm_bindgen_test]
fn test_handles_conflicts_between_positive_and_negative_values_correctly() {
    assert_eq!(tw_merge("-m-2 m-auto"), "m-auto");
    assert_eq!(tw_merge("top-12 -top-69"), "-top-69");
}

#[wasm_bindgen_test]
fn test_handles_conflicts_across_groups_with_negative_values_correctly() {
    assert_eq!(tw_merge("-right-1 inset-x-1"), "inset-x-1");
    assert_eq!(
        tw_merge("hover:focus:-right-1 focus:hover:inset-x-1"),
        "focus:hover:inset-x-1"
    );
}
