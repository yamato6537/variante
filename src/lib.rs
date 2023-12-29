// Copyright 2023 yamato6537.
//
// Licensed under either of MIT (https://opensource.org/license/mit/) or Apache 2.0 (https://opensource.org/license/apache-2-0/), at your option.
// This file may not be copied, modified, or distributed except according to those terms.

//! Statically-verified `enum` variant names as strings.
//!
//! The [`variant!`] macro evaluates to a variant's name after verifying that the variant exists on the specified `enum`.
//! The type must be in scope, and the variant must be visible (accessible) at the point of invocation of the macro.
//!
//! # Usage
//!
//! Invoke the macro as `variant!(Variant @ Enum)`:
//! ```
//! use variante::*;
//!
//! enum Enum {
//!     Foo,
//!     Bar(u8),
//!     Baz { x: i32 },
//! }
//!
//! let foo = variant!(Foo @ Enum);
//! assert_eq!(foo, "Foo");
//!
//! let bar = variant!(Bar @ Enum);
//! assert_eq!(bar, "Bar");
//! 
//! let baz = variant!(Baz @ Enum);
//! assert_eq!(baz, "Baz")
//! ```
//!
//! A variant that is not on the specified `enum` or a type that is not in scope will cause a compilation error:
//! ```compile_fail
//! // This fails because there is no variant named "Hoge" on "Enum"
//! let hoge = variant!(Hoge @ Enum);
//!
//! // This fails because their is no struct named "NonExistent"
//! let foo = variant!(Foo @ NonExistent);
//! ```
//!
//! # Generics
//!
//! [`variant!`] also works with generic types, as long as concrete type parameters are provided:
//! ```
//! use variante::*;
//!
//! enum GenericEnum<T, U> {
//!     Foo(T),
//!     Bar(U),
//! }
//!
//! let foo = variant!(Foo @ GenericEnum<(), ()>);
//! assert_eq!(foo, "Foo");
//!
//! // Any type can be used for the type parameter(s)
//! let bar = variant!(Bar @ GenericEnum<i32, i64>);
//! assert_eq!(bar, "Bar");
//! ```
//!
//! # Paths
//!
//! That's right, [`variant!`] also works with path syntax:
//! ```
//! use variante::*;
//!
//! mod fuga {
//!     pub enum Enum<T> {
//!         Foo(T), // Must be pub so that it is visible at the point of invocation
//!     }
//! }
//!
//! let foo = variant!(Foo @ fuga::Enum<()>);
//! assert_eq!(foo, "Foo");
//! ```
//!
//! # Dependencies
//!
//! This crate is completely dependency-free.
//! `#[no_std]` is also supported by default.

#![warn(missing_docs)]
#![no_std]

/// Checks for the presence of a variant on a `enum` at compile-time and returns the field's name as a `&'static str`.
///
/// Invoked as `variant!(Variant @ Enum)`, this macro verifies that a variant named `Variant` exists on the type `Enum`, and is visible at the point of invocation.
/// It then stringifies the variant name, evaluating to `"Variant"`.
///
/// Refer to the [crate-level documentation](https://docs.rs/variante) for more information.
#[macro_export]
macro_rules! variant {
    ( $variant:ident @ $($enumeration:ident)::+ $(<$($generics:ty),+>)? ) => {{
        #[allow(unused)]
        fn assert_variant_exists( on: $($enumeration)::+ $(<$($generics),+>)? ) {
            matches!(on, $($enumeration)::+ :: $variant {..});
        }
        ::core::stringify!($variant)
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn normal() {
        #[allow(unused)]
        enum Enum {
            Foo,
            Bar(u8),
            Baz { x: i32 }
        }

        let foo = variant!(Foo @ Enum);
        assert_eq!(foo, "Foo");

        let bar = variant!(Bar @ Enum);
        assert_eq!(bar, "Bar");

        let baz = variant!(Baz @ Enum);
        assert_eq!(baz, "Baz");
    }

    #[test]
    fn generic() {
        #[allow(unused)]
        enum GenericEnum<T, U> {
            Foo(T),
            Bar(U),
        }

        let foo = variant!(Foo @ GenericEnum<(), ()>);
        assert_eq!(foo, "Foo");

        let bar = variant!(Bar @ GenericEnum<(), ()>);
        assert_eq!(bar, "Bar");
    }

    // #[test]
    // fn fails() {
    //     enum Enum {
    //         Foo,
    //     }
    //     let _ = variant!(Bar @ Enum);
    // }
}