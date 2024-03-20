#![cfg(target_arch = "wasm32")]

extern crate tailwind_merge_rs;

use tailwind_merge_rs::tw_merge;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_tw_merge() {
    assert_eq!(
        tw_merge(String::from("mix-blend-normal mix-blend-multiply")),
        "mix-blend-multiply"
    );
    assert_eq!(tw_merge(String::from("h-10 h-min")), "h-min");
    assert_eq!(
        tw_merge(String::from("stroke-black stroke-1")),
        "stroke-black stroke-1"
    );
    assert_eq!(tw_merge(String::from("stroke-2 stroke-[3]")), "stroke-[3]");
    assert_eq!(
        tw_merge(String::from("outline-black outline-1")),
        "outline-black outline-1"
    );
    assert_eq!(
        tw_merge(String::from("grayscale-0 grayscale-[50%]")),
        "grayscale-[50%]"
    );
    assert_eq!(tw_merge(String::from("grow grow-[2]")), "grow-[2]");
}
