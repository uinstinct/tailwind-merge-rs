#![cfg(target_arch = "wasm32")]

extern crate tailwind_merge_rs;

use tailwind_merge_rs::tw_merge;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_handles_conflicts_across_class_groups_correctly() {
    assert_eq!(
        tw_merge(String::from("inset-1 inset-x-1")),
        "inset-1 inset-x-1"
    );
    assert_eq!(tw_merge(String::from("inset-x-1 inset-1")), "inset-1");
    assert_eq!(
        tw_merge(String::from("inset-x-1 left-1 inset-1")),
        "inset-1"
    );
    assert_eq!(
        tw_merge(String::from("inset-x-1 inset-1 left-1")),
        "inset-1 left-1"
    );
    assert_eq!(
        tw_merge(String::from("inset-x-1 right-1 inset-1")),
        "inset-1"
    );
    assert_eq!(
        tw_merge(String::from("inset-x-1 right-1 inset-x-1")),
        "inset-x-1"
    );
    assert_eq!(
        tw_merge(String::from("inset-x-1 right-1 inset-y-1")),
        "inset-x-1 right-1 inset-y-1"
    );
    assert_eq!(
        tw_merge(String::from("right-1 inset-x-1 inset-y-1")),
        "inset-x-1 inset-y-1"
    );
    assert_eq!(
        tw_merge(String::from("inset-x-1 hover:left-1 inset-1")),
        "hover:left-1 inset-1"
    );
}

#[wasm_bindgen_test]
fn test_ring_and_shadow_classes_do_not_create_conflict() {
    assert_eq!(tw_merge(String::from("ring shadow")), "ring shadow");
    assert_eq!(
        tw_merge(String::from("ring-2 shadow-md")),
        "ring-2 shadow-md"
    );
    assert_eq!(tw_merge(String::from("shadow ring")), "shadow ring");
    assert_eq!(
        tw_merge(String::from("shadow-md ring-2")),
        "shadow-md ring-2"
    );
}

#[wasm_bindgen_test]
fn test_touch_classes_create_conflicts_correctly() {
    assert_eq!(
        tw_merge(String::from("touch-pan-x touch-pan-right")),
        "touch-pan-right"
    );
    assert_eq!(
        tw_merge(String::from("touch-none touch-pan-x")),
        "touch-pan-x"
    );
    assert_eq!(
        tw_merge(String::from("touch-pan-x touch-none")),
        "touch-none"
    );
    assert_eq!(
        tw_merge(String::from("touch-pan-x touch-pan-y touch-pinch-zoom")),
        "touch-pan-x touch-pan-y touch-pinch-zoom"
    );
    assert_eq!(
        tw_merge(String::from(
            "touch-manipulation touch-pan-x touch-pan-y touch-pinch-zoom"
        )),
        "touch-pan-x touch-pan-y touch-pinch-zoom"
    );
    assert_eq!(
        tw_merge(String::from(
            "touch-pan-x touch-pan-y touch-pinch-zoom touch-auto"
        )),
        "touch-auto"
    );
}

#[wasm_bindgen_test]
fn test_line_clamp_classes_create_conflicts_correctly() {
    assert_eq!(
        tw_merge(String::from("overflow-auto inline line-clamp-1")),
        "line-clamp-1"
    );
    assert_eq!(
        tw_merge(String::from("line-clamp-l overflow-auto inline")),
        "line-clamp-l overflow-auto inline"
    );
}
