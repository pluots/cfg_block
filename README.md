# cfg_block

A simple library for applying procedural macros to a block, for easier `const`
values and more readable code. Grab the docs here <https://docs.rs/cfg_block>.

Here's a simple example for defining variables based on platform:

```rs
use cfg_block::cfg_block;

cfg_block!{
    #[cfg(target_family = "unix")] {
        const PLATFORM: &str = "posix !";
        const MY_NUMBER: u8 = 5;
    }
    #[cfg(target_family = "windows")] {
        const PLATFORM: &str = "window !";
        const MY_NUMBER: u16 = 20;
    }
    #[cfg(target_family = "wasm")] {
        const PLATFORM: &str = "web !";
        const MY_NUMBER: i32 = -5;
    }
}

// Assuming this test runs on linux/macos...
assert_eq!(PLATFORM, "posix !");
assert_eq!(MY_NUMBER, 5);
```

The above example demonstrates using `#[cfg()]`, but it should work for any
procedural macro. Behind the scenes, it just inserts the macro attribute before
every item in the block, and removes the block wrapper.

This macro also supports a simple if/else configuration:

```rs
cfg_block!{
    if #[cfg(mips)] {
        const STR_A: &str = "where did you get this processor";
        const STR_B: &str = "mips just makes a good passing doctest";
    } else {
        const STR_A: &str = "good!";
        const STR_B: &str = "better!";
    }
}

assert_eq(STR_A, "good!");
assert_eq(STR_B, "better!");
```

Please note that unlike the general syntax, this if/else syntax only works with
`#[cfg(something)]` (it just replaces it with `#[cfg(not(something))]`).

## Links

Link to `crates.io` page: <https://crates.io/crates/cfg_block>

Get the docs here: <https://docs.rs/cfg_block>

Source repository: <https://github.com/pluots/cfg_block>
