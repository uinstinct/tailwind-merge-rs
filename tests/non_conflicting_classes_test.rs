#![cfg(target_arch = "wasm32")]

extern crate tailwind_merge;

use tailwind_merge::tw_merge;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_merges_non_conflicting_classes_correctly() {
    assert_eq!(
        tw_merge("border-t border-white/10"),
        "border-t border-white/10"
    );
    assert_eq!(
        tw_merge("border-t border-white"),
        "border-t border-white"
    );
    assert_eq!(
        tw_merge("text-3.5xl text-black"),
        "text-3.5xl text-black"
    );
}
