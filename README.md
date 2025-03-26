# tcolour

[![Crates.io](https://img.shields.io/crates/v/tcolour.svg)](https://crates.io/crates/tcolour) [![Docs.rs](https://docs.rs/tcolour/badge.svg)](https://docs.rs/tcolour) [![License](https://img.shields.io/crates/l/tcolour.svg)](https://crates.io/crates/tcolour)

**tcolour** is just a crate that I have made for a project I am working on. It is designed to work with [ratatui](https://ratatui.rs/) and also has capabilities to convert to and from [nalgebra](https://nalgebra.org/)'s `Vector4<f64>`. I intend to test this crate to some degree but not with extreme rigour, feel free to use.

## Examples

**Important Note:** this crate uses `auto_ops` to define all operators for borrowed/owned iterations of `f64 (operator) Colour` (and vice versa) along with `Colour (operator) Colour`. Every operator is component-wise and all are invariant over the alpha channel. Similarly, the operators do not normalise or clamp the channels (or clean `NaN` values). To get around these facts, you can either: use the `nalgebra` feature if you have it in your project or follow a suggested tip from this example:

### Alpha channel operating

```rust
use tcolour::Colour;

let red = Colour::red(1.0);
let blue = Colour::blue(1.0);
// if the alpha channel was also added, it would be 2.
let purple = red + blue;

let translucent_red = Colour::red(1.0).with_alpha(0.3);
let almost_opaque_blue = Colour::blue(1.0).with_alpha(0.7);
// we add the alpha channels manually
let solid_purple = (translucent_red + almost_opaque_blue)
    .with_alpha(translucent_red.a + almost_opaque_blue.a);
```

### Cleaning a colour

```rust
// RGBA(1, 1, 1, 1)
let white = Colour::grey(1.0);

// RGBA(0, 0, 0, 0)
let transparent = Colour::transparent();

// RGBA(NaN, NaN, NaN, NaN)
let faulty_colour = white / transparent;

/*
    `.cleaned(&self) -> Colour` will replace all NaN
    values with 1f64 as this is usually the intended
    effect for colour division by zero.
*/
assert_eq!(white, faulty_colour.cleaned());

// ...alternatively convert the values to black
assert_eq!(Colour::grey(0.0), faulty_colour.map_rgba(|v|
    if value.is_nan() { 0f64 } else { v }
));
```

Another note: there are many "iterator" like functions for `Colour` but not an implementation for `iter()` (yet). This is because I felt it best for, at least my code, to keep within the `Colour` type and not be converting between `iter` to `vec` to then `Colour::try_from` and so on. For this functionality you can use one of the many `Into<>` bindings to convert from `Colour` to `[f64; 4]` or `Vec<f64>` or `(f64, f64, f64, f64)` or `[u8; 3]` (ignores the alpha channel) and so on. For how to use `Colour`, or how I use it at least, feel free to read, fork, and whatever else the AGPL3.0 license permits you to do with the source code (pretty much anything).

## Features

The features of this crate just allow you to choose which implementations are valid.

---

### [ratatui](https::/ratatui.rs/)

I have abused the fact that American English and British English spell color/colour differently to differentiate between [ratatui](https://ratatui.rs/)'s `Color` and **tcolour**'s `Colour` but feel free to do `use tcolour::Colour as TColor` or something.

```rust
use tcolour::Colour;
use approx::assert_relative_eq;

let colour: Colour = Colour::from(ratatui::style::Color::Rgb(10, 15, 12));
// [0, 255] ⊂ ℕ to [0, 1] ⊂ ℝ
assert_relative_eq!(colour, Colour::from_u8(10, 15, 12));
```

### [nalgebra](https://nalgebra.org/)

```rust
use tcolour::Colour;

let colour: Colour = Colour::from(nalgebra::Vector4::new(0f64, 1f64, 1f64, 0.5f64));
// (x, y, z, w) = (r, g, b, a)
assert_eq!(colour, Colour::new(0f64, 1f64, 1f64, 0.5f64))
```

### [approx](https://docs.rs/approx)

**RelativeEq**, **AbsDiffEq** and **UlpsEq** are all defined for `Colour` by this crate.

```rust
let colour: Colour = Colour::solid(1f64, 1f64, 1f64);
/*
    Some mathematics that might result in floating point errors,
    especially true after using something like `Colour::blend()` 
*/
let new_colour = colour + // ...
/*
    Some differet mathematics that might result in floating point 
    errors but should result in the same value as `new_colour`
*/
let newer_colour = colour + // ... 
assert_relative_eq!(new_colour, newer_colour);
```

---

By default I have enabled [ratatui](https://ratatui.rs/) as this is the intended target for this crate and [approx](https://docs.rs/approx) due to blending having a fair possibility in producing some floating point errors and the use of `std::f64`.

```toml
features = ["naglebra", "ratatui", "approx"]

default-features = ["ratatui", "approx"]
```
