#![cfg(target_arch = "wasm32")]

extern crate tailwind_merge_rs;

use tailwind_merge_rs::tw_merge;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_basic_arbitrary_variants() {
    assert_eq!(
        tw_merge("[&>*]:underline [&>*]:line-through"),
        "[&>*]:line-through"
    );
    assert_eq!(
        tw_merge("[&>*]:underline [&>*]:line-through [&_div]:line-through"
        ),
        "[&>*]:line-through [&_div]:line-through"
    );
    assert_eq!(
        tw_merge("supports-[display:grid]:flex supports-[display:grid]:grid"
        ),
        "supports-[display:grid]:grid"
    );
}

#[wasm_bindgen_test]
fn test_arbitrary_variants_with_modifiers() {
    assert_eq!(
        tw_merge("dark:lg:hover:[&>*]:underline dark:lg:hover:[&>*]:line-through"
        ),
        "dark:lg:hover:[&>*]:line-through"
    );
    assert_eq!(
        tw_merge("dark:lg:hover:[&>*]:underline dark:hover:lg:[&>*]:line-through"
        ),
        "dark:hover:lg:[&>*]:line-through"
    );
    assert_eq!(
        tw_merge("hover:[&>*]:underline [&>*]:hover:line-through"
        ),
        "hover:[&>*]:underline [&>*]:hover:line-through"
    );
    assert_eq!(
        tw_merge("hover:dark:[&>*]:underline dark:hover:[&>*]:underline dark:[&>*]:hover:line-through"
        ),
        "dark:hover:[&>*]:underline dark:[&>*]:hover:line-through"
    );
}

#[wasm_bindgen_test]
fn test_arbitrary_variants_with_complex_syntax() {
    assert_eq!(tw_merge("[@media_screen{@media(hover:hover)}]:underline [@media_screen{@media(hover:hover)}]:line-through"), "[@media_screen{@media(hover:hover)}]:line-through");
    assert_eq!(tw_merge("hover:[@media_screen{@media(hover:hover)}]:underline hover:[@media_screen{@media(hover:hover)}]:line-through"), "hover:[@media_screen{@media(hover:hover)}]:line-through");
}

#[wasm_bindgen_test]
fn test_arbitrary_variants_with_attribute_selectors() {
    assert_eq!(
        tw_merge("[&[data-open]]:underline [&[data-open]]:line-through"
        ),
        "[&[data-open]]:line-through"
    );
}

#[ignore]
#[wasm_bindgen_test]
fn test_multiple_arbitrary_variants() {
    assert_eq!(
        tw_merge("[&>*]:[&_div]:underline [&>*]:[&_div]:line-through"
        ),
        "[&>*]:[&_div]:line-through"
    );
    assert_eq!(
        tw_merge("[&>*]:[&_div]:underline [&_div]:[&>*]:line-through"
        ),
        "[&>*]:[&_div]:underline [&_div]:[&>*]:line-through"
    );
    assert_eq!(tw_merge("hover:dark:[&>*]:focus:disabled:[&_div]:underline dark:hover:[&>*]disabled:focus:[&_div]line-through"), "dark:hover:[&>]*disabled:focus:[&_div]line-through");
    assert_eq!(tw_merge("hover:dark:[&>]*focus:[&_div]disabled:underline dark:hover:[&>]*disabled:focus:[&_div]line-through"), "hover:dark:[&>]*focus:[&_div]disabled:underline dark:hover:[&>]*disabled:focus:[&_div]line-through");
}

#[ignore]
#[wasm_bindgen_test]
fn test_arbitrary_variants_with_arbitrary_properties() {
    assert_eq!(
        tw_merge("[&>]*[color:red] [&>]*[color:blue]"),
        "[&>]*[color:blue]"
    );
    assert_eq!(tw_merge("[&[data-foo][data-bar]not([data-baz])nod:noa[color:red] [&[data-foo][data-bar]not([data-baz])noa:nod[color:blue]"), "[&[data-foo][data-bar]not([data-baz])noa:nod[color:blue]");
}
