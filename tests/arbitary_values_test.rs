#![cfg(target_arch = "wasm32")]

extern crate tailwind_merge_rs;

use tailwind_merge_rs::tw_merge;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_handles_simple_conflicts_with_arbitrary_values_correctly() {
    assert_eq!(tw_merge("m-[2px] m-[10px]"), "m-[10px]");
    assert_eq!(
        tw_merge("m-[2px] m-[11svmin] m-[12in] m-[13lvi] m-[14vb] m-[15vmax] m-[16mm] m-[17%] m-[18em] m-[19px] m-[10dvh]"),
        "m-[10dvh]"
    );
    assert_eq!(
        tw_merge("h-[10px] h-[11cqw] h-[12cqh] h-[13cqi] h-[14cqb] h-[15cqmin] h-[16cqmax]"
        ),
        "h-[16cqmax]"
    );
    assert_eq!(tw_merge("z-20 z-[99]"), "z-[99]");
    assert_eq!(tw_merge("my-[2px] m-[10rem]"), "m-[10rem]");
    assert_eq!(
        tw_merge("cursor-pointer cursor-[grab]"),
        "cursor-[grab]"
    );
    assert_eq!(
        tw_merge("m-[2px] m-[calc(100%-var(--arbitrary))]"),
        "m-[calc(100%-var(--arbitrary))]"
    );
    assert_eq!(
        tw_merge("m-[2px] m-[length:var(--mystery-var)]"),
        "m-[length:var(--mystery-var)]"
    );
    assert_eq!(
        tw_merge("opacity-10 opacity-[0.025]"),
        "opacity-[0.025]"
    );
    assert_eq!(
        tw_merge("scale-75 scale-[1.7]"),
        "scale-[1.7]"
    );
    assert_eq!(
        tw_merge("brightness-90 brightness-[1.75]"),
        "brightness-[1.75]"
    );

    // Handling of value `0`
    assert_eq!(
        tw_merge("min-h-[0.5px] min-h-[0]"),
        "min-h-[0]"
    );
    assert_eq!(
        tw_merge("text-[0.5px] text-[color:0]"),
        "text-[0.5px] text-[color:0]"
    );
    assert_eq!(
        tw_merge("text-[0.5px] text-[--my-0]"),
        "text-[0.5px] text-[--my-0]"
    );
}

#[wasm_bindgen_test]
fn test_handles_arbitrary_length_conflicts_with_labels_and_modifiers_correctly() {
    assert_eq!(
        tw_merge("hover:m-[2px] hover:m-[length:var(--c)]"),
        "hover:m-[length:var(--c)]"
    );
    assert_eq!(
        tw_merge("hover:focus:m-[2px] focus:hover:m-[length:var(--c)]"
        ),
        "focus:hover:m-[length:var(--c)]"
    );
    assert_eq!(
        tw_merge("border-b border-[color:rgb(var(--color-gray-500-rgb)/50%)]"
        ),
        "border-b border-[color:rgb(var(--color-gray-500-rgb)/50%)]"
    );
    assert_eq!(
        tw_merge("border-[color:rgb(var(--color-gray-500-rgb)/50%)] border-b"
        ),
        "border-[color:rgb(var(--color-gray-500-rgb)/50%)] border-b"
    );
    assert_eq!(
        tw_merge("border-b border-[color:rgb(var(--color-gray-500-rgb)/50%)] border-some-coloooor"
        ),
        "border-b border-some-coloooor"
    );
}

#[wasm_bindgen_test]
fn test_handles_complex_arbitrary_value_conflicts_correctly() {
    assert_eq!(
        tw_merge("grid-rows-[1fr,auto] grid-rows-2"),
        "grid-rows-2"
    );
    assert_eq!(
        tw_merge("grid-rows-[repeat(20,minmax(0,1fr))] grid-rows-3"
        ),
        "grid-rows-3"
    );
}

#[wasm_bindgen_test]
fn test_handles_ambiguous_arbitrary_values_correctly() {
    assert_eq!(
        tw_merge("mt-2 mt-[calc(theme(fontSize.4xl)/1.125)]"),
        "mt-[calc(theme(fontSize.4xl)/1.125)]"
    );
    assert_eq!(
        tw_merge("p-2 p-[calc(theme(fontSize.4xl)/1.125)_10px]"),
        "p-[calc(theme(fontSize.4xl)/1.125)_10px]"
    );
    assert_eq!(
        tw_merge("mt-2 mt-[length:theme(someScale.someValue)]"),
        "mt-[length:theme(someScale.someValue)]"
    );
    assert_eq!(
        tw_merge("mt-2 mt-[theme(someScale.someValue)]"),
        "mt-[theme(someScale.someValue)]"
    );
    assert_eq!(
        tw_merge("text-2xl text-[length:theme(someScale.someValue)]"
        ),
        "text-[length:theme(someScale.someValue)]"
    );
    assert_eq!(
        tw_merge("text-2xl text-[calc(theme(fontSize.4xl)/1.125)]"
        ),
        "text-[calc(theme(fontSize.4xl)/1.125)]"
    );
    assert_eq!(
        tw_merge("bg-cover bg-[percentage:30%] bg-[length:200px_100px]"
        ),
        "bg-[length:200px_100px]"
    );
    assert_eq!(
        tw_merge("bg-none bg-[url(.)] bg-[image:.] bg-[url:.] bg-[linear-gradient(.)] bg-gradient-to-r"
        ),
        "bg-gradient-to-r"
    );
}
