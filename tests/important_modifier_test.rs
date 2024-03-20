#![cfg(target_arch = "wasm32")]

extern crate tailwind_merge;

use tailwind_merge::tw_merge;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_merges_tailwind_classes_with_important_modifier_correctly() {
    assert_eq!(
        tw_merge("!font-medium !font-bold"),
        "!font-bold"
    );
    assert_eq!(
        tw_merge("!font-medium !font-bold font-thin"),
        "!font-bold font-thin"
    );
    assert_eq!(
        tw_merge("!right-2 !-inset-x-px"),
        "!-inset-x-px"
    );
    assert_eq!(
        tw_merge("focus:!inline focus:!block"),
        "focus:!block"
    );
}
