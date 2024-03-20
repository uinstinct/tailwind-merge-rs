#![cfg(target_arch = "wasm32")]

extern crate tailwind_merge_rs;

use tailwind_merge_rs::tw_merge;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_merges_standalone_classes_from_same_group_correctly() {
    assert_eq!(tw_merge(String::from("inline block")), "block");
    assert_eq!(
        tw_merge(String::from("hover:block hover:inline")),
        "hover:inline"
    );
    assert_eq!(
        tw_merge(String::from("hover:block hover:block")),
        "hover:block"
    );
    assert_eq!(
        tw_merge(String::from(
            "inline hover:inline focus:inline hover:block hover:focus:block"
        )),
        "inline focus:inline hover:block hover:focus:block"
    );
    assert_eq!(
        tw_merge(String::from("underline line-through")),
        "line-through"
    );
    assert_eq!(
        tw_merge(String::from("line-through no-underline")),
        "no-underline"
    );
}
