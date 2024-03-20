#![cfg(target_arch = "wasm32")]

extern crate tailwind_merge;

use tailwind_merge::tw_merge;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_supports_tailwind_css_v3_3_features() {
    assert_eq!(
        tw_merge("text-red text-lg/7 text-lg/8"),
        "text-red text-lg/8"
    );
    assert_eq!(
        tw_merge(
            vec![
                "start-0 start-1",
                "end-0 end-1",
                "ps-0 ps-1 pe-0 pe-1",
                "ms-0 ms-1 me-0 me-1",
                "rounded-s-sm rounded-s-md rounded-e-sm rounded-e-md",
                "rounded-ss-sm rounded-ss-md rounded-ee-sm rounded-ee-md"
            ]
            .join(" ").as_str()
        ),
        "start-1 end-1 ps-1 pe-1 ms-1 me-1 rounded-s-md rounded-e-md rounded-ss-md rounded-ee-md",
    );
    assert_eq!(
        tw_merge("start-0 end-0 inset-0 ps-0 pe-0 p-0 ms-0 me-0 m-0 rounded-ss rounded-es rounded-s"
        ),
        "inset-0 p-0 m-0 rounded-s"
    );
    assert_eq!(
        tw_merge("hyphens-auto hyphens-manual"),
        "hyphens-manual"
    );
    assert_eq!(
        tw_merge("from-0% from-10% from-[12.5%] via-0% via-10% via-[12.5%] to-0% to-10% to-[12.5%]"
        ),
        "from-[12.5%] via-[12.5%] to-[12.5%]",
    );
    assert_eq!(
        tw_merge("from-0% from-red"),
        "from-0% from-red"
    );
    assert_eq!(
        tw_merge("list-image-none list-image-[url(./my-image.png)] list-image-[var(--value)]"
        ),
        "list-image-[var(--value)]"
    );
    assert_eq!(
        tw_merge("caption-top caption-bottom"),
        "caption-bottom"
    );
    assert_eq!(
        tw_merge("line-clamp-2 line-clamp-none line-clamp-[10]"),
        "line-clamp-[10]"
    );
    assert_eq!(
        tw_merge("delay-150 delay-0 duration-150 duration-0"),
        "delay-0 duration-0"
    );
    assert_eq!(
        tw_merge("justify-normal justify-center justify-stretch"
        ),
        "justify-stretch"
    );
    assert_eq!(
        tw_merge("content-normal content-center content-stretch"
        ),
        "content-stretch"
    );
    assert_eq!(
        tw_merge("whitespace-nowrap whitespace-break-spaces"),
        "whitespace-break-spaces"
    );
}

#[wasm_bindgen_test]
fn test_supports_tailwind_css_v3_4_features() {
    assert_eq!(
        tw_merge("h-svh h-dvh w-svw w-dvw"),
        "h-dvh w-dvw"
    );
    assert_eq!(
        tw_merge(
            "has-[[data-potato]]:p-1 has-[[data-potato]]:p-2 group-has-[:checked]:grid group-has-[:checked]:flex"),
        
        "has-[[data-potato]]:p-2 group-has-[:checked]:flex"
    );
    assert_eq!(
        tw_merge("text-wrap text-pretty"),
        "text-pretty"
    );
    assert_eq!(
        tw_merge("w-5 h-3 size-10 w-12"),
        "size-10 w-12"
    );
    assert_eq!(
        tw_merge("grid-cols-2 grid-cols-subgrid grid-rows-5 grid-rows-subgrid"
        ),
        "grid-cols-subgrid grid-rows-subgrid"
    );
    assert_eq!(
        tw_merge("min-w-0 min-w-50 min-w-px max-w-0 max-w-50 max-w-px"
        ),
        "min-w-px max-w-px"
    );
    assert_eq!(
        tw_merge("forced-color-adjust-none forced-color-adjust-auto"
        ),
        "forced-color-adjust-auto"
    );
    assert_eq!(
        tw_merge("appearance-none appearance-auto"),
        "appearance-auto"
    );
    assert_eq!(
        tw_merge("float-start float-end clear-start clear-end"),
        "float-end clear-end"
    );
    assert_eq!(
        tw_merge("*:p-10 *:p-20 hover:*:p-10 hover:*:p-20"),
        "*:p-20 hover:*:p-20"
    );
}
