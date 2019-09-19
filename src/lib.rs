//! Convert values to `bool`, kind of like C, eh?
//!
//! [`Eh`] roughly follows the implicit conversion rules for [C to `_Bool`][C]
//! or [C++ to `bool`][C++], but Rust requires an explicit conversion. Integer
//! `0`, floating-point `0.0`, and null pointers are `false`, and all other
//! values are `true`.
//!
//! [`Eh`]: trait.Eh.html
//! [C]: https://en.cppreference.com/w/c/language/conversion#Boolean_conversion
//! [C++]: https://en.cppreference.com/w/cpp/language/implicit_conversion#Boolean_conversions
//!
//! As a Rust-specific extension, this is also implemented for `Option<T>` and
//! `Result<T, E>`. It returns `true` when the `?` operator would unwrap a `T`
//! value, and `false` when `?` would cause an early return.
//!
//! ## Examples
//!
//! Boolean values just return themselves.
//!
//! ```
//! use eh::Eh;
//! assert!(true.eh());
//! assert!(true.eh().eh());
//! assert!(!(false.eh()));
//! ```
//!
//! Integers are `true` for all non-zero values.
//!
//! ```
//! use eh::Eh;
//! use std::i32;
//! assert!(1.eh());
//! assert!((-1).eh());
//! assert!(i32::MIN.eh());
//! assert!(i32::MAX.eh());
//! assert!(!(0.eh()));
//! ```
//!
//! Wrapping integers follow the same rules.
//!
//! ```
//! use eh::Eh;
//! use std::num::Wrapping;
//! assert!(Wrapping(1).eh());
//! assert!(!(Wrapping(0).eh()));
//! ```
//!
//! Floats are also `true` for all non-zero values -- including NaN!
//!
//! ```
//! use eh::Eh;
//! use std::f64;
//! assert!(1.0.eh());
//! assert!((-1.0).eh());
//! assert!(f64::EPSILON.eh());
//! assert!(f64::INFINITY.eh());
//! assert!(f64::NAN.eh());
//! assert!(!(0.0.eh()));
//! assert!(!(-0.0).eh());
//! ```
//!
//! Raw pointers are `true` for any non-null value, without dereferencing.
//!
//! ```
//! use eh::Eh;
//! use std::ptr;
//! assert!((&0 as *const i32).eh());
//! assert!((&mut 0 as *mut i32).eh());
//! assert!(ptr::NonNull::<i32>::dangling().as_ptr().eh());
//! assert!(!ptr::null::<i32>().eh());
//! assert!(!ptr::null_mut::<i32>().eh());
//! ```
//!
//! Options are `true` for any `Some` value, and `false` for `None`.
//!
//! ```
//! use eh::Eh;
//! assert!(Some(0).eh());
//! assert!(Some(1).eh());
//! assert!(!None::<i32>.eh());
//! ```
//!
//! Results are `true` for any `Ok` value, and `false` for any `Err` value.
//!
//! ```
//! use eh::Eh;
//! assert!(Ok::<i32, i32>(0).eh());
//! assert!(Ok::<i32, i32>(1).eh());
//! assert!(!Err::<i32, i32>(0).eh());
//! assert!(!Err::<i32, i32>(1).eh());
//! ```
//!
//! ## Exclusions
//!
//! `Eh` does not implement further boolean conversions of other languages,
//! especially since they're not universal. For example:
//!
//! - JavaScript converts NaN to `false`, different than C and C++ (and `eh`).
//! - JavaScript converts empty `[]` and `{}` to `true`, but in Python they're `false`.
//! - Many languages convert empty strings to `false` and non-empty strings to `true`,
//!   but `"0"` is also `false` in Perl and PHP.
//!
//! ## About
//!
//! The name is a play on the [Canadian "eh"][eh], turning a declarative
//! sentence into a question.
//!
//! [eh]: https://en.wikipedia.org/wiki/Eh#Canada

#![no_std]

/// Convert values to `bool`, kind of like C, eh?
pub trait Eh {
    fn eh(&self) -> bool;
}

impl Eh for bool {
    #[inline]
    fn eh(&self) -> bool {
        *self
    }
}

macro_rules! int_eh {
    ($($T:ty),*) => {$(
        impl Eh for $T {
            #[inline]
            fn eh(&self) -> bool {
                *self != 0
            }
        }
    )*}
}
int_eh! { i8, i16, i32, i64, i128, isize }
int_eh! { u8, u16, u32, u64, u128, usize }

macro_rules! float_eh {
    ($($T:ty),*) => {$(
        impl Eh for $T {
            #[inline]
            fn eh(&self) -> bool {
                *self != 0.0
            }
        }
    )*}
}
float_eh! { f32, f64 }

impl<T: ?Sized> Eh for *const T {
    #[inline]
    fn eh(&self) -> bool {
        !self.is_null()
    }
}

impl<T: ?Sized> Eh for *mut T {
    #[inline]
    fn eh(&self) -> bool {
        !self.is_null()
    }
}

impl<T: Eh> Eh for core::num::Wrapping<T> {
    #[inline]
    fn eh(&self) -> bool {
        self.0.eh()
    }
}

impl<T> Eh for Option<T> {
    #[inline]
    fn eh(&self) -> bool {
        self.is_some()
    }
}

impl<T, E> Eh for Result<T, E> {
    #[inline]
    fn eh(&self) -> bool {
        self.is_ok()
    }
}
