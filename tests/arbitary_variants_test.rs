#![cfg(target_arch = "wasm32")]

extern crate tailwind_merge_rs;

use tailwind_merge_rs::tw_merge;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_basic_arbitrary_variants() {
    assert_eq!(
        tw_merge(String::from("[&>*]:underline [&>*]:line-through")),
        "[&>*]:line-through"
    );
    assert_eq!(
        tw_merge(String::from(
            "[&>*]:underline [&>*]:line-through [&_div]:line-through"
        )),
        "[&>*]:line-through [&_div]:line-through"
    );
    assert_eq!(
        tw_merge(String::from(
            "supports-[display:grid]:flex supports-[display:grid]:grid"
        )),
        "supports-[display:grid]:grid"
    );
}

#[wasm_bindgen_test]
fn test_arbitrary_variants_with_modifiers() {
    assert_eq!(
        tw_merge(String::from(
            "dark:lg:hover:[&>*]:underline dark:lg:hover:[&>*]:line-through"
        )),
        "dark:lg:hover:[&>*]:line-through"
    );
    assert_eq!(
        tw_merge(String::from(
            "dark:lg:hover:[&>*]:underline dark:hover:lg:[&>*]:line-through"
        )),
        "dark:hover:lg:[&>*]:line-through"
    );
    assert_eq!(
        tw_merge(String::from(
            "hover:[&>*]:underline [&>*]:hover:line-through"
        )),
        "hover:[&>*]:underline [&>*]:hover:line-through"
    );
    assert_eq!(
        tw_merge(String::from(
            "hover:dark:[&>*]:underline dark:hover:[&>*]:underline dark:[&>*]:hover:line-through"
        )),
        "dark:hover:[&>*]:underline dark:[&>*]:hover:line-through"
    );
}

#[wasm_bindgen_test]
fn test_arbitrary_variants_with_complex_syntax() {
    assert_eq!(tw_merge(String::from("[@media_screen{@media(hover:hover)}]:underline [@media_screen{@media(hover:hover)}]:line-through")), "[@media_screen{@media(hover:hover)}]:line-through");
    assert_eq!(tw_merge(String::from("hover:[@media_screen{@media(hover:hover)}]:underline hover:[@media_screen{@media(hover:hover)}]:line-through")), "hover:[@media_screen{@media(hover:hover)}]:line-through");
}

#[wasm_bindgen_test]
fn test_arbitrary_variants_with_attribute_selectors() {
    assert_eq!(
        tw_merge(String::from(
            "[&[data-open]]:underline [&[data-open]]:line-through"
        )),
        "[&[data-open]]:line-through"
    );
}

#[ignore]
#[wasm_bindgen_test]
fn test_multiple_arbitrary_variants() {
    assert_eq!(
        tw_merge(String::from(
            "[&>*]:[&_div]:underline [&>*]:[&_div]:line-through"
        )),
        "[&>*]:[&_div]:line-through"
    );
    assert_eq!(
        tw_merge(String::from(
            "[&>*]:[&_div]:underline [&_div]:[&>*]:line-through"
        )),
        "[&>*]:[&_div]:underline [&_div]:[&>*]:line-through"
    );
    assert_eq!(tw_merge(String::from("hover:dark:[&>*]:focus:disabled:[&_div]:underline dark:hover:[&>*]disabled:focus:[&_div]line-through")), "dark:hover:[&>]*disabled:focus:[&_div]line-through");
    assert_eq!(tw_merge(String::from("hover:dark:[&>]*focus:[&_div]disabled:underline dark:hover:[&>]*disabled:focus:[&_div]line-through")), "hover:dark:[&>]*focus:[&_div]disabled:underline dark:hover:[&>]*disabled:focus:[&_div]line-through");
}

#[ignore]
#[wasm_bindgen_test]
fn test_arbitrary_variants_with_arbitrary_properties() {
    assert_eq!(
        tw_merge(String::from("[&>]*[color:red] [&>]*[color:blue]")),
        "[&>]*[color:blue]"
    );
    assert_eq!(tw_merge(String::from("[&[data-foo][data-bar]not([data-baz])nod:noa[color:red] [&[data-foo][data-bar]not([data-baz])noa:nod[color:blue]")), "[&[data-foo][data-bar]not([data-baz])noa:nod[color:blue]");
}
