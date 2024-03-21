# tailwind-merge-rs

This is an implementation of the [tailwind-merge](https://github.com/dcastil/tailwind-merge) library in rust.

## Installation

Add the following line in your `Cargo.toml` dependencies

```toml
tailwind-merge = { git = "https://github.com/uinstinct/tailwind-merge-rs.git" }
```

## Usage

```rs
use tailwind_merge::tw_merge;

tw_merge("px-2 py-1 bg-red hover:bg-dark-red" + " " + "p-3 bg-[#B91C1C]")
// → "hover:bg-dark-red p-3 bg-[#B91C1C]"
```