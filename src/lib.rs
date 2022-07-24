//! A simple library for applying procedural macros to a block. Simple example:
//! 
//! ```
//! cfg_block!{
//!     if #[cfg(mips)] {
//!         const STR_A: &str = "where did you get this processor";
//!         const STR_B: &str = "mips just makes a good passing doctest";
//!     } else {
//!         const STR_A: &str = "good!";
//!         const STR_B: &str = "better!";
//!     }
//! }
//! 
//! assert_eq(STR_A, "good!");
//! assert_eq(STR_B, "better!");
//! ```
//! 
//! Most of the documentation is contained at [`cfg_block`], please see there
//! for more details.


/// Allow applying inner procedural macros to a `{}` block
/// 
/// This is 
/// 
/// # Examples
/// 
/// Basic example handling differnt 
/// 
/// ```
/// use cfg_block::cfg_block;
/// 
/// cfg_block!{
///     #[cfg(target_family = "unix")] {
///         const PLATFORM: &str = "posix !";
///         const MY_NUMBER: u8 = 5;
///     }
///     #[cfg(target_family = "windows")] {
///         const PLATFORM: &str = "window !";
///         const MY_NUMBER: u16 = 20;
///     }
///     #[cfg(target_family = "wasm")] {
///         const PLATFORM: &str = "web !";
///         const MY_NUMBER: i32 = -5;
///     }
/// }
/// 
/// // Assuming this test runs on linux/macos...
/// assert_eq!(PLATFORM, "posix !");
/// assert_eq!(MY_NUMBER, 5);
/// ```
/// 
/// Or, with the if/else syntax (only works for `cfg` macros):
/// 
/// ```
/// use cfg_block::cfg_block;
/// 
/// cfg_block!{
///     if #[cfg(mips)] {
///         const STR_A: &str = "where did you get this processor";
///         const STR_B: &str = "mips just makes a good passing doctest";
///     } else {
///         const STR_A: &str = "good!";
///         const STR_B: &str = "better!";
///     }
/// }
/// 
/// assert_eq(STR_A, "good!");
/// assert_eq(STR_B, "better!");
/// ```
/// 
/// Currently, if/else statements need to be in a separate `cfg_block!{}` from
/// the default syntax.
/// 
/// It is also possible to put anything that rust considers an `item` inside the
/// block. Note that this does not include `let` bindings.
/// 
/// ```
/// use cfg_block::cfg_block;
/// 
/// cfg_block!{
///     if #[cfg(mips)] {
///         const STR_A: &str = "where did you get this processor";
///     } else {
///         const STR_A: &str = "good!";
///     }
/// 
///     if #[cfg(not(mips))] {
///         assert_eq!(STR_A, "good!");
///     } else {}
/// }
/// ```
#[macro_export]
macro_rules! cfg_block {
    ($( if #[cfg($meta:meta)] {$($item:item)*} else {$($item_f:item)*} )*) => {
        $(
        $(
            #[cfg($meta)]
            $item
        )*
        $(
            #[cfg(not($meta))]
            $item_f
        )*
        )*
    };
    ($( #[$meta:meta] {$($item:item)*} )*) => {
        $($(
            #[$meta]
            $item
        )*)*
    }
}

#[cfg(test)]
mod tests {
    //! Note: these tests use the (probably correct) assumption that we're not
    //! testing on a mips processor
    use super::*;

    cfg_block!{
        #[cfg(not(mips))] {
            const SINGLE_A: &str = "something a";
            const SINGLE_B: &str = "something b";
        }
    }

    #[test]
    fn single_validation() {
        assert_eq!(SINGLE_A, "something a");
        assert_eq!(SINGLE_B, "something b");
    }


    cfg_block!{
        if #[cfg(mips)] {
            const ELSE_A: &str = "nothing a";
            const ELSE_B: &str = "nothing b";
        } else {
            const ELSE_A: &str = "something a";
            const ELSE_B: &str = "something b";
        }
    }

    #[test]
    fn else_validation() {
        assert_eq!(ELSE_A, "something a");
        assert_eq!(ELSE_B, "something b");
    }


    #[test]
    fn inside_item_validation() {
        cfg_block!{
            if #[cfg(mips)] {
                assert_eq!(ELSE_A, "nothing a");
                assert_eq!(ELSE_B, "nothing b");
            } else {
                assert_eq!(ELSE_A, "something a");
                assert_eq!(ELSE_B, "something b");
            }
        }
    }

    // Currently does not work
    // #[test]
    // fn inside_validation() {
    //     // Test within a function block
    //     cfg_block!{
    //         #[cfg(mips)] {
    //             let inside_a: &str = "nothing a";
    //             let inside_b: &str = "nothing b";
    //         } else {
    //             let inside_a: &str = "something a";
    //             let inside_b: &str = "something b";
    //         }
            
    //     }
    //     assert_eq!(inside_a, "something a");
    //     assert_eq!(inside_b, "something b");
    // }
}
