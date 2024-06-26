#![cfg(target_arch = "wasm32")]

extern crate tailwind_merge;

use tailwind_merge::tw_merge;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_handles_pseudo_variants_conflicts_properly() {
    assert_eq!(tw_merge("empty:p-2 empty:p-3"), "empty:p-3");
    assert_eq!(
        tw_merge("hover:empty:p-2 hover:empty:p-3"),
        "hover:empty:p-3"
    );
    assert_eq!(
        tw_merge("read-only:p-2 read-only:p-3"),
        "read-only:p-3"
    );
}

#[wasm_bindgen_test]
fn test_handles_pseudo_variant_group_conflicts_properly() {
    assert_eq!(
        tw_merge("group-empty:p-2 group-empty:p-3"),
        "group-empty:p-3"
    );
    assert_eq!(
        tw_merge("peer-empty:p-2 peer-empty:p-3"),
        "peer-empty:p-3"
    );
    assert_eq!(
        tw_merge("group-empty:p-2 peer-empty:p-3"),
        "group-empty:p-2 peer-empty:p-3"
    );
    assert_eq!(
        tw_merge("hover:group-empty:p-2 hover:group-empty:p-3"),
        "hover:group-empty:p-3"
    );
    assert_eq!(
        tw_merge("group-read-only:p-2 group-read-only:p-3"),
        "group-read-only:p-3"
    );
}
