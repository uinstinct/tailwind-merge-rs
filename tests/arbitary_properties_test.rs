#![cfg(target_arch = "wasm32")]

extern crate tailwind_merge_rs;

use tailwind_merge_rs::tw_merge;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_arbitrary_property_conflicts() {
    assert_eq!(
        tw_merge("[paint-order:markers] [paint-order:normal]"),
        "[paint-order:normal]"
    );
    assert_eq!(
        tw_merge(
            "[paint-order:markers] [--my-var:2rem] [paint-order:normal] [--my-var:4px]"
        ),
        "[paint-order:normal] [--my-var:4px]"
    );
}

#[wasm_bindgen_test]
fn test_arbitrary_property_conflicts_with_modifiers() {
    assert_eq!(
        tw_merge(
            "[paint-order:markers] hover:[paint-order:normal]"
        ),
        "[paint-order:markers] hover:[paint-order:normal]"
    );
    assert_eq!(
        tw_merge(
            "hover:[paint-order:markers] hover:[paint-order:normal]"
        ),
        "hover:[paint-order:normal]"
    );
    assert_eq!(
        tw_merge(
            "hover:focus:[paint-order:markers] focus:hover:[paint-order:normal]"
        ),
        "focus:hover:[paint-order:normal]"
    );
    assert_eq!(
        tw_merge(
            "[paint-order:markers] [paint-order:normal] [--my-var:2rem] lg:[--my-var:4px]"
        ),
        "[paint-order:normal] [--my-var:2rem] lg:[--my-var:4px]"
    );
}

#[wasm_bindgen_test]
fn test_complex_arbitrary_property_conflicts() {
    assert_eq!(
        tw_merge(
            "[-unknown-prop:::123:::] [-unknown-prop:url(https://hi.com)]"
        ),
        "[-unknown-prop:url(https://hi.com)]"
    );
}

#[wasm_bindgen_test]
fn test_important_modifier() {
    assert_eq!(
        tw_merge("![some:prop] [some:other]"),
        "![some:prop] [some:other]"
    );
    assert_eq!(
        tw_merge(
            "![some:prop] [some:other] [some:one] ![some:another]"
        ),
        "[some:one] ![some:another]"
    );
}
