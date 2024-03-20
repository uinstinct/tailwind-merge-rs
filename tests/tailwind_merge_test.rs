#![cfg(target_arch = "wasm32")]

extern crate tailwind_merge;

use tailwind_merge::tw_merge;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_tw_merge() {
    assert_eq!(
        tw_merge("mix-blend-normal mix-blend-multiply"),
        "mix-blend-multiply"
    );
    assert_eq!(tw_merge("h-10 h-min"), "h-min");
    assert_eq!(
        tw_merge("stroke-black stroke-1"),
        "stroke-black stroke-1"
    );
    assert_eq!(tw_merge("stroke-2 stroke-[3]"), "stroke-[3]");
    assert_eq!(
        tw_merge("outline-black outline-1"),
        "outline-black outline-1"
    );
    assert_eq!(
        tw_merge("grayscale-0 grayscale-[50%]"),
        "grayscale-[50%]"
    );
    assert_eq!(tw_merge("grow grow-[2]"), "grow-[2]");
}
