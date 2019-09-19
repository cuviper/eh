# eh

[![Latest Version]][crates.io] [![docs]][docs.rs]

Convert values to `bool`, kind of like C, eh?

[`Eh`] roughly follows the implicit conversion rules for [C to `_Bool`][C]
or [C++ to `bool`][C++], but Rust requires an explicit conversion. Integer
`0`, floating-point `0.0`, and null pointers are `false`, and all other
values are `true`.

[`Eh`]: https://docs.rs/eh/0.1/eh/trait.Eh.html
[C]: https://en.cppreference.com/w/c/language/conversion#Boolean_conversion
[C++]: https://en.cppreference.com/w/cpp/language/implicit_conversion#Boolean_conversions

As a Rust-specific extension, this is also implemented for `Option<T>` and
`Result<T, E>`. It returns `true` when the `?` operator would unwrap a `T`
value, and `false` when `?` would cause an early return.

## Exclusions

`Eh` does not implement further boolean conversions of other languages,
especially since they're not universal. For example:

- JavaScript converts NaN to `false`, different than C and C++ (and `eh`).
- JavaScript converts empty `[]` and `{}` to `true`, but in Python they're `false`.
- Many languages convert empty strings to `false` and non-empty strings to `true`,
  but `"0"` is also `false` in Perl and PHP.

## About

The name is a play on the [Canadian "eh"][eh], turning a declarative
sentence into a question.

[eh]: https://en.wikipedia.org/wiki/Eh#Canada

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `eh` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[Latest Version]: https://img.shields.io/crates/v/eh.svg
[crates.io]: https://crates.io/crates/eh
[docs]: https://docs.rs/eh/badge.svg
[docs.rs]: https://docs.rs/eh/

