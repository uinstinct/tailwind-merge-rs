#![cfg(target_arch = "wasm32")]

extern crate tailwind_merge;

use tailwind_merge::tw_merge;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_handles_color_conflicts_properly() {
    assert_eq!(tw_merge("bg-grey-5 bg-hotpink"), "bg-hotpink");
    assert_eq!(
        tw_merge("hover:bg-grey-5 hover:bg-hotpink"),
        "hover:bg-hotpink"
    );
    assert_eq!(
        tw_merge("stroke-[hsl(350_80%_0%)] stroke-[10px]"),
        "stroke-[hsl(350_80%_0%)] stroke-[10px]"
    );
}
