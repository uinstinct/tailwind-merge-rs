#![cfg(target_arch = "wasm32")]

extern crate tailwind_merge;

use tailwind_merge::tw_merge;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_merges_standalone_classes_from_same_group_correctly() {
    assert_eq!(tw_merge("inline block"), "block");
    assert_eq!(
        tw_merge("hover:block hover:inline"),
        "hover:inline"
    );
    assert_eq!(
        tw_merge("hover:block hover:block"),
        "hover:block"
    );
    assert_eq!(
        tw_merge("inline hover:inline focus:inline hover:block hover:focus:block"
        ),
        "inline focus:inline hover:block hover:focus:block"
    );
    assert_eq!(
        tw_merge("underline line-through"),
        "line-through"
    );
    assert_eq!(
        tw_merge("line-through no-underline"),
        "no-underline"
    );
}
