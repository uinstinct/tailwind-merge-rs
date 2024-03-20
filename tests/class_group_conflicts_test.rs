#![cfg(target_arch = "wasm32")]

extern crate tailwind_merge;

use tailwind_merge::tw_merge;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_merges_classes_from_same_group_correctly() {
    assert_eq!(
        tw_merge("overflow-x-auto overflow-x-hidden"),
        "overflow-x-hidden"
    );
    assert_eq!(
        tw_merge("basis-full basis-auto"),
        "basis-auto"
    );
    assert_eq!(tw_merge("w-full w-fit"), "w-fit");
    assert_eq!(
        tw_merge("overflow-x-auto overflow-x-hidden overflow-x-scroll"
        ),
        "overflow-x-scroll"
    );
    assert_eq!(
        tw_merge("overflow-x-auto hover:overflow-x-hidden overflow-x-scroll"
        ),
        "hover:overflow-x-hidden overflow-x-scroll"
    );
    assert_eq!(
        tw_merge("overflow-x-auto hover:overflow-x-hidden hover:overflow-x-auto overflow-x-scroll"
        ),
        "hover:overflow-x-auto overflow-x-scroll"
    );
    assert_eq!(
        tw_merge("col-span-1 col-span-full"),
        "col-span-full"
    );
}

#[wasm_bindgen_test]
fn test_merges_classes_from_font_variant_numeric_section_correctly() {
    assert_eq!(
        tw_merge("lining-nums tabular-nums diagonal-fractions"),
        "lining-nums tabular-nums diagonal-fractions"
    );
    assert_eq!(
        tw_merge("normal-nums tabular-nums diagonal-fractions"),
        "tabular-nums diagonal-fractions"
    );
    assert_eq!(
        tw_merge("tabular-nums diagonal-fractions normal-nums"),
        "normal-nums"
    );
    assert_eq!(
        tw_merge("tabular-nums proportional-nums"),
        "proportional-nums"
    );
}
