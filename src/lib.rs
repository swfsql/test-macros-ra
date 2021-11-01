//! `cargo check` runs "Ok", but RA errors.
//!
//! `cargo doc --no-deps` works "Ok":
//! Has an empty module `usages`,
//! the macros `ok_macro` and `my_macro`,
//! and two structs `CreatedByOkMacro` and `CreatedByMyMacro`.
//!
//! env info:
//!
//! rustup show: 1.55.0-x86_64-unknown-linux-gnu
//! RA: v0.2.801
//! VsCode settings:
//! "rust-analyzer.procMacro.enable": true
//! "rust-analyzer.experimental.procAttrMacros": true

// macro that will replace the item by a macro definition, like `ok_macro`
use macros::my_macro_attr;

// just a macro example
#[macro_use]
mod macro_def {
    #[macro_export]
    macro_rules! ok_macro {
        () => {
            pub struct CreatedByOkMacro;
        };
    }
}

// this will replace the struct item with a macro
// named `my_macro` that works similarly to `ok_macro`,
// except it will define a struct named `CreatedByMyMacro`
#[my_macro_attr]
struct WillBeReplaced;

// check/RA: "Ok" on both, since they were `#[macro_export]`
ok_macro!();
my_macro!();

/// I have a module that wants to use my own root macros:
pub mod usages {
    #[allow(unused_imports)]
    mod usage_from_macro_use {
        //! Accessible because of `#[macro_use]` up to the
        //! macro definitions.

        // check/RA: "Ok"
        use ok_macro;

        // check: "OK"
        use my_macro;
        //  ^^^^^^^^ RA error: unresolved import
    }

    #[allow(unused_imports)]
    mod usage_from_crate {
        //! This is just a test showing that using `crate::`
        //! doesn't work for `my_macro`, even on cargo check.
        //!

        // check/RA: "Ok"
        use crate::ok_macro;

        // use crate::my_macro;
        //     ^^^^^^^^^^^^^^^ check error: macro-expanded `macro_export` macros
        //                     from the current crate cannot be referred to
        //                     by absolute paths
    }
}
