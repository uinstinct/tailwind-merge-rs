#![cfg(target_arch = "wasm32")]

extern crate tailwind_merge_rs;

use tailwind_merge_rs::tw_merge;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_handles_color_conflicts_properly() {
    assert_eq!(tw_merge(String::from("bg-grey-5 bg-hotpink")), "bg-hotpink");
    assert_eq!(
        tw_merge(String::from("hover:bg-grey-5 hover:bg-hotpink")),
        "hover:bg-hotpink"
    );
    assert_eq!(
        tw_merge(String::from("stroke-[hsl(350_80%_0%)] stroke-[10px]")),
        "stroke-[hsl(350_80%_0%)] stroke-[10px]"
    );
}
