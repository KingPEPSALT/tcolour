use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use auto_ops::{impl_op_ex, impl_op_ex_commutative};
use std::convert::TryFrom;

pub enum BlendMode {
    /// Do not blend, just compose the colours
    Normal,

    Multiply,
    Divide,
    Addition,
    Subtract,

    Screen,
    Overlay,
    HardLight,
    SoftLight,

    Darken,
    Lighten,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Colour {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

#[cfg(feature="approx")]
impl AbsDiffEq for Colour {
    type Epsilon = f64;
    fn default_epsilon() -> Self::Epsilon {
        1e-6
    }
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.all_rgba_with(*other, |v_self, v_other| {
            f64::abs_diff_eq(&v_self, &v_other, epsilon)
        })
    }
}

#[cfg(feature="approx")]
impl RelativeEq for Colour {
    fn default_max_relative() -> Self::Epsilon {
        1e-6
    }
    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.all_rgba_with(*other,|v_self, v_other| {
            f64::relative_eq(&v_self, &v_other, epsilon, max_relative)
        })
    }
}

#[cfg(feature="approx")]
impl UlpsEq for Colour {
    fn default_max_ulps() -> u32 {
        4
    }
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.all_rgba_with(*other, |v_self, v_other| {
            f64::ulps_eq(&v_self, &v_other, epsilon, max_ulps)
        })
    }
}

// ---------- Implemented operators for Colour ----------
impl_op_ex_commutative!(+|a: &Colour, b: &f64| -> Colour {
    Colour::new(a.r + b, a.g + b, a.b + b, a.a)
});
impl_op_ex_commutative!(*|a: &Colour, b: &f64| -> Colour {
    Colour::new(a.r * b, a.g * b, a.b * b, a.a)
});
impl_op_ex!(+|a: &Colour, b: &Colour| -> Colour {
    Colour::new(a.r + b.r, a.g + b.g, a.b + b.b, a.a)
});
impl_op_ex!(*|a: &Colour, b: &Colour| -> Colour {
    Colour::new(a.r * b.r, a.g * b.g, a.b * b.b, a.a)
});

impl_op_ex!(-|a: &Colour, b: &f64| -> Colour { Colour::new(a.r - b, a.g - b, a.b - b, a.a) });
impl_op_ex!(-|b: &f64, a: &Colour| -> Colour { Colour::new(b - a.r, b - a.g, b - a.b, a.a) });
impl_op_ex!(-|a: &Colour, b: &Colour| -> Colour {
    Colour::new(a.r - b.r, a.g - b.g, a.b - b.b, a.a)
});

impl_op_ex!(/|a: &Colour, b: &f64| -> Colour {
    Colour::new(a.r / b, a.g / b, a.b / b, a.a)
});
impl_op_ex!(/|b: &f64, a: &Colour| -> Colour {
    Colour::new(b/a.r, b/a.g, b/a.b, a.a)
});
impl_op_ex!(/|a: &Colour, b: &Colour| -> Colour {
    Colour::new(a.r/b.r, a.g/b.g, a.b/b.b, a.a)
});

impl_op_ex!(-|a: &Colour| -> Colour { a.inverted() });

impl Colour {
    /// Creates a new Colour
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self { r, g, b, a }
    }

    /// Creates a solid greyscale colour
    pub fn grey(grey: f64) -> Self {
        Self::solid(grey, grey, grey)
    }

    /// Creates a solid red colour
    pub fn red(red: f64) -> Self {
        Self::solid(red, 0f64, 0f64)
    }

    /// Creates a solid green colour
    pub fn green(green: f64) -> Self {
        Self::solid(0f64, green, 0f64)
    }

    /// Creates a solid blue colour
    pub fn blue(blue: f64) -> Self {
        Self::solid(0f64, 0f64, blue)
    }

    /// Creates a colour by normalising `u8` values with
    /// `alpha = 1`
    pub fn from_u8(r: u8, g: u8, b: u8) -> Self {
        Self::solid(r as f64 / 255f64, g as f64 / 255f64, b as f64 / 255f64)
    }

    /// Creates a colour by normalising `u8` values
    pub fn from_u8_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::from_u8(r, g, b).with_alpha(a as f64 / 255f64)
    }

    /// Converts the colour to a standard `u8` colour
    ///
    /// Note: does NOT composite the alpha into the colour,
    /// for alpha retrieval, use `.as_u8_rgba()`
    pub fn as_u8(&self) -> (u8, u8, u8) {
        (
            (self.r * 255f64) as u8,
            (self.g * 255f64) as u8,
            (self.b * 255f64) as u8,
        )
    }

    pub fn as_u8_rgba(&self) -> (u8, u8, u8, u8) {
        (
            (self.r * 255f64) as u8,
            (self.g * 255f64) as u8,
            (self.b * 255f64) as u8,
            (self.a * 255f64) as u8,
        )
    }

    /// Returns a Colour with alpha as `1`.
    pub fn solid(r: f64, g: f64, b: f64) -> Self {
        Self::new(r, g, b, 1f64)
    }

    /// Returns the Colour `RGBA(0, 0, 0, 0)`.
    pub fn transparent() -> Self {
        Self::new(0f64, 0f64, 0f64, 0f64)
    }

    /// Sets alpha to the given value and returns `Self`.
    pub fn with_alpha(mut self, alpha: f64) -> Self {
        self.a = alpha;
        self
    }

    /// Sets red to the given value and returns `Self`.
    pub fn with_red(mut self, red: f64) -> Self {
        self.r = red;
        self
    }

    /// Sets blue to the given value and returns `Self`.
    pub fn with_blue(mut self, blue: f64) -> Self {
        self.b = blue;
        self
    }

    /// Sets green to the given value and returns `Self`.
    pub fn with_green(mut self, green: f64) -> Self {
        self.g = green;
        self
    }

    /// Change each value of `self` in place, excluding alpha
    ///
    /// # Example
    /// ```
    /// use approx::assert_relative_eq;
    /// use tcolour::Colour;
    ///
    /// let mut colour = Colour::new(1.0, 1.0, 0.2, 0.4);
    /// colour.apply(|v| if *v < 0.5 { *v += 0.5 });
    ///
    /// assert_relative_eq!(colour, Colour::new(1.0, 1.0, 0.7, 0.4));
    /// ```
    pub fn apply<F: FnMut(&mut f64)>(&mut self, closure: F) {
        [&mut self.r, &mut self.g, &mut self.b]
            .into_iter()
            .for_each(closure)
    }

    /// Change each value of `self` in place, including alpha
    ///
    /// # Example
    /// ```
    /// use approx::assert_relative_eq;
    /// use tcolour::Colour;
    ///
    /// let mut colour = Colour::new(1.0, 1.0, 0.2, 0.4);
    /// colour.apply_rgba(|v| if *v < 0.5 { *v += 0.5 });
    ///
    /// assert_relative_eq!(colour, Colour::new(1.0, 1.0, 0.7, 0.9));
    /// ```
    pub fn apply_rgba<F: FnMut(&mut f64)>(&mut self, closure: F) {
        [&mut self.r, &mut self.g, &mut self.b, &mut self.a]
            .into_iter()
            .for_each(closure)
    }

    /// Maps each value of `self` to another, excluding alpha
    ///
    /// # Example
    /// ```
    /// use approx::assert_relative_eq;
    /// use tcolour::Colour;
    ///
    /// let colour = Colour::new(1.0, 1.0, 0.2, 0.4);
    /// let mapped = colour.map(|v| if v < 0.5 { v + 0.5 } else { v });
    ///
    /// assert_relative_eq!(mapped, Colour::new(1.0, 1.0, 0.7, 0.4));
    /// ```
    pub fn map<F: Fn(f64) -> f64>(&self, closure: F) -> Self {
        Colour::new(closure(self.r), closure(self.g), closure(self.b), self.a)
    }

    /// Maps each value of `self` to another, including alpha
    ///
    /// # Example
    /// ```
    /// use approx::assert_relative_eq;
    /// use tcolour::Colour;
    ///
    /// let colour = Colour::new(1.0, 1.0, 0.2, 0.4);
    /// let mapped = colour.map_rgba(|v| if v < 0.5 { v + 0.5 } else { v });
    ///
    /// assert_relative_eq!(mapped, Colour::new(1.0, 1.0, 0.7, 0.9));
    /// ```
    pub fn map_rgba<F: Fn(f64) -> f64>(&self, closure: F) -> Self {
        Colour::new(
            closure(self.r),
            closure(self.g),
            closure(self.b),
            closure(self.a),
        )
    }

    /// Maps each value of `self` alongside values of another colour
    /// but keeps the alpha value of `self`
    ///
    /// # Example
    /// ```
    /// use approx::assert_relative_eq;
    /// use tcolour::Colour;
    ///
    /// let a = Colour::new(1.0, 1.0, 1.0, 1.0);
    /// let b = Colour::new(0.6, 0.6, 0.4, 0.5);
    /// let c = a.map_with(b, |a_value, b_value| (a_value+b_value)/2f64);
    ///
    /// assert_relative_eq!(c, Colour::new(0.8, 0.8, 0.7, 1.0));
    /// ```
    pub fn map_with<F: Fn(f64, f64) -> f64>(&self, other: Self, closure: F) -> Self {
        Colour::new(
            closure(self.r, other.r),
            closure(self.g, other.g),
            closure(self.b, other.b),
            self.a,
        )
    }

    /// Maps each value of `self` alongside values of another colour
    /// including the alpha value
    ///
    /// # Example
    /// ```
    /// use approx::assert_relative_eq;
    /// use tcolour::Colour;
    ///
    /// let a = Colour::new(1.0, 1.0, 1.0, 1.0);
    /// let b = Colour::new(0.6, 0.6, 0.4, 0.5);
    /// let c = a.map_rgba_with(b, |a_value, b_value| (a_value+b_value)/2f64);
    ///
    /// assert_relative_eq!(c, Colour::new(0.8, 0.8, 0.7, 0.75));
    /// ```
    pub fn map_rgba_with<F: Fn(f64, f64) -> f64>(&self, other: Self, closure: F) -> Self {
        Colour::new(
            closure(self.r, other.r),
            closure(self.g, other.g),
            closure(self.b, other.b),
            closure(self.a, other.a),
        )
    }

    pub fn all<F: Fn(f64) -> bool>(&self, predicate: F) -> bool {
        predicate(self.r) && predicate(self.g) && predicate(self.b)
    }

    pub fn all_rgba<F: Fn(f64) -> bool>(&self, predicate: F) -> bool {
        self.all(&predicate) && predicate(self.a)
    }

    pub fn all_with<F: Fn(f64, f64) -> bool>(&self, other: Self, predicate: F) -> bool {
        predicate(self.r, other.r) && predicate(self.g, other.g) && predicate(self.b, other.b)
    }

    pub fn all_rgba_with<F: Fn(f64, f64) -> bool>(&self, other: Self, predicate: F) -> bool {
        self.all_with(other, &predicate) && predicate(self.a, other.a)
    }
    /// Blends two colours together using one of the many blend modes
    /// and then composites the blended colour onto the base colour
    /// using alpha compositing.
    ///
    /// The values are not necessarily normalised on return if `self`
    /// or `other` are not normalised.
    ///
    /// This treats `self` as the base layer and other as the
    /// blend layer, use `.blend_onto()` to swap this around
    pub fn blend(&self, other: Self, blend_mode: BlendMode) -> Self {
        // Blend the RGB values first
        let blended = match blend_mode {
            BlendMode::Normal => other,
            BlendMode::Addition => self + other,
            BlendMode::Subtract => self - other,
            BlendMode::Multiply => self * other,
            BlendMode::Divide => self / other,
            BlendMode::Darken => self.map_with(other, |base, blend| base.min(blend)),
            BlendMode::Lighten => self.map_with(other, |base, blend| base.max(blend)),
            BlendMode::Screen => -(-self * -other),
            BlendMode::Overlay => self.map_with(other, |base, blend| {
                if base < 0.5f64 {
                    2f64 * blend * base
                } else {
                    1f64 - 2f64 * (1f64 - base) * (1f64 - blend)
                }
            }),
            BlendMode::HardLight => other.blend(*self, BlendMode::Overlay),
            BlendMode::SoftLight => self * -(-other * -other) + -self * other,
        }
        .cleaned();
        // Compose the colours with the alpha
        let alpha_composite = other.a + self.a * (1f64 - other.a);
        ((blended * other.a + self * self.a * (1f64 - other.a)) / (alpha_composite))
            .with_alpha(alpha_composite)
    }

    /// Blends two colours together using one of the many blend modes
    /// and then composites the blended colour onto the base colour
    /// using alpha compositing.
    ///
    /// This treats `other` as the base layer and `self` as the
    /// blend layer, use `.blend()` to swap this around
    pub fn blend_onto(self, other: Self, blend_mode: BlendMode) -> Self {
        other.blend(self, blend_mode)
    }

    /// Alpha compose the two colours together. This is the same
    /// as blending with `BlendMode::Normal`
    /// ```
    /// use approx::assert_relative_eq;
    /// use tcolour::{Colour, BlendMode};
    ///
    /// let blend_layer = Colour::grey(0.5).with_alpha(0.3);
    /// let base_layer = Colour::red(1.0);
    ///
    /// assert_relative_eq!(base_layer.compose(blend_layer), base_layer.blend(blend_layer, BlendMode::Normal));
    /// assert_relative_eq!(base_layer.compose(blend_layer), blend_layer.compose_onto(base_layer));
    /// ```
    pub fn compose(&self, other: Self) -> Self {
        self.blend(other, BlendMode::Normal)
    }

    /// Alpha compose the two colours together. This is the same
    /// as blending with `BlendMode::Normal`
    /// ```
    /// use approx::assert_relative_eq;
    /// use tcolour::{Colour, BlendMode};
    ///
    /// let blend_layer = Colour::grey(0.5).with_alpha(0.3);
    /// let base_layer = Colour::red(1.0);
    ///
    /// assert_relative_eq!(blend_layer.compose_onto(base_layer), blend_layer.blend_onto(base_layer, BlendMode::Normal));
    /// assert_relative_eq!(blend_layer.compose_onto(base_layer), base_layer.compose(blend_layer));
    /// ```
    pub fn compose_onto(&self, other: Self) -> Self {
        self.blend_onto(other, BlendMode::Normal)
    }

    /// Linearly interpolate between two colours
    pub fn lerp(&self, other: Self, t: f64) -> Self {
        self + (other - self) * t
    }

    /// Gets the highest channel
    pub fn max_channel(&self) -> f64 {
        self.r.max(self.g.max(self.b.max(self.a)))
    }

    /// Gets the lowest channel
    pub fn min_channel(&self) -> f64 {
        self.r.min(self.g.min(self.b.min(self.a)))
    }

    /// All `NaN`, `inf` and [subnormal](https://en.wikipedia.org/wiki/Subnormal_number) 
    /// values become `1`, (the usual intended result for divisions by `0`). 
    /// 
    /// Returns the result.
    /// 
    /// # Example
    /// 
    /// ```
    /// use tcolour::Colour;
    /// 
    /// let colour = Colour::grey(0.5);
    /// let invalid_colour = colour / 0.0;
    /// 
    /// assert_eq!(invalid_colour.cleaned(), Colour::grey(1.0));
    /// ```
    pub fn cleaned(&self) -> Self {
        self.map_rgba(|v| if !v.is_normal() { 1f64 } else { v })
    }

    /// All `NaN`, `inf` and [subnormal](https://en.wikipedia.org/wiki/Subnormal_number) 
    /// values become `1`, (the usual intended result for divisions by `0`). 
    /// 
    /// Modifies `self` in place.
    ///    
    /// # Example
    /// 
    /// ```
    /// use tcolour::Colour;
    /// 
    /// let mut colour = Colour::grey(0.5) / 0.0;
    /// colour.clean();
    /// 
    /// assert_eq!(colour, Colour::grey(1.0));
    /// ```
    pub fn clean(&mut self) {
        self.apply_rgba(|v| {
            if !v.is_normal() {
                *v = 1f64
            };
        });
    }

    /// Clamps all values to between `[0, 1]`, returns the result.
    pub fn clamped(&self) -> Self {
        self.map_rgba(|v| v.clamp(0f64, 1f64))
    }

    /// Clamps all values to between `[0, 1]`, modifies `self` in place.
    pub fn clamp(&mut self) {
        self.apply_rgba(|v| *v = v.clamp(0f64, 1f64));
    }

    /// Inverts the Colour flipping values from
    /// `1` to `0` and vice versa by `1 - value`.
    /// This is the same as `-self`.
    ///
    /// Returns the inverted Colour of `self`.
    ///
    /// # Example
    /// ```
    /// use approx::assert_relative_eq;
    /// use tcolour::Colour;
    ///
    /// let colour = Colour::new(0.8, 0.3, 1.0, 0.9);
    /// let inverted = colour.inverted();
    ///
    /// assert_relative_eq!(inverted, Colour::new(0.2, 0.7, 0.0, 0.9));
    /// ```
    pub fn inverted(self) -> Self {
        1f64 - self
    }

    /// Inverts the Colour flipping values from
    /// `1` to `0` and vice versa by `1 - value`.
    /// This is the same as `self = -self`.
    ///
    /// Modifies `self` in place.
    ///
    /// # Example
    /// ```
    /// use approx::assert_relative_eq;
    /// use tcolour::Colour;
    ///
    /// let mut colour = Colour::new(0.8, 0.3, 1.0, 0.9);
    /// colour.invert();
    ///
    /// assert_relative_eq!(colour, Colour::new(0.2, 0.7, 0.0, 0.9));
    /// ```
    pub fn invert(&mut self) {
        *self = self.inverted()
    }

    /// Normalises the values using the highest and lowest
    /// values within the colour, including alpha. Only
    /// changes `self` if there is either a value above
    /// or below `1` or `0`. If the highest colour is less 
    /// than `1`, it is treated as `1` and if the lowest colour
    /// is more than `0` it is treated as `0`
    ///
    /// Modifies `self` in place.
    ///
    /// # Example
    /// ```
    /// use approx::assert_relative_eq;
    /// use tcolour::Colour;
    ///
    /// let mut colour = Colour::grey(1.5).with_alpha(1.5);
    /// colour.normalise();
    ///
    /// assert_relative_eq!(colour, Colour::grey(1.0));
    ///
    ///
    /// let mut another_colour = Colour::grey(0.5);
    /// another_colour.normalise();
    ///
    /// assert_relative_eq!(another_colour, Colour::grey(0.5));
    /// ```
    pub fn normalise(&mut self) {
        let (max, min) = (self.max_channel().max(1f64), self.min_channel().min(0f64));
        if min >= 0f64 && max <= 1f64 {
            return;
        }
        self.apply_rgba(|v| *v = (*v - min) / (max - min));
    }

    /// Normalises the values using the highest and lowest
    /// values within the colour, including alpha. Only
    /// returns a value different from `self` if there is
    /// either a value above or below `1` or `0`
    ///
    /// Returns the result.
    ///
    /// # Example
    /// ```
    /// use approx::assert_relative_eq;
    /// use tcolour::Colour;
    ///
    /// let colour = Colour::grey(1.5).with_alpha(1.5).normalised();
    ///
    /// assert_relative_eq!(colour, Colour::grey(1.0));
    ///
    ///
    /// let another_colour = Colour::grey(0.5).normalised();
    ///
    /// assert_relative_eq!(another_colour, Colour::grey(0.5));
    /// ```
    pub fn normalised(mut self) -> Self {
        self.normalise();
        self
    }
}

impl From<[f64; 3]> for Colour {
    fn from(value: [f64; 3]) -> Self {
        Colour::solid(value[0], value[1], value[2])
    }
}
impl From<[f64; 4]> for Colour {
    fn from(value: [f64; 4]) -> Self {
        Colour::new(value[0], value[1], value[2], value[3])
    }
}
impl From<(f64, f64, f64)> for Colour {
    fn from((r, g, b): (f64, f64, f64)) -> Self {
        Colour::solid(r, g, b)
    }
}
impl From<(f64, f64, f64, f64)> for Colour {
    fn from((r, g, b, a): (f64, f64, f64, f64)) -> Self {
        Colour::new(r, g, b, a)
    }
}

impl From<[u8; 3]> for Colour {
    fn from(value: [u8; 3]) -> Self {
        Colour::from_u8(value[0], value[1], value[2])
    }
}
impl From<[u8; 4]> for Colour {
    fn from(value: [u8; 4]) -> Self {
        Colour::from_u8_rgba(value[0], value[1], value[2], value[3])
    }
}
impl From<(u8, u8, u8)> for Colour {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Colour::from_u8(r, g, b)
    }
}
impl From<(u8, u8, u8, u8)> for Colour {
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Self {
        Colour::from_u8_rgba(r, g, b, a)
    }
}

impl TryFrom<&[f64]> for Colour {
    type Error = String;
    fn try_from(value: &[f64]) -> Result<Self, Self::Error> {
        if value.len() > 4 {
            Err("There are too many elements.".to_string())
        } else if value.len() < 3 {
            Err("There are not enough elements.".to_string())
        } else if value.len() == 3 {
            Ok(Colour::solid(value[0], value[1], value[2]))
        } else {
            Ok(Colour::new(value[0], value[1], value[2], value[3]))
        }
    }
}

impl TryFrom<Vec<f64>> for Colour {
    type Error = String;
    fn try_from(value: Vec<f64>) -> Result<Self, Self::Error> {
        if value.len() > 4 {
            Err("There are too many elements.".to_string())
        } else if value.len() < 3 {
            Err("There are not enough elements.".to_string())
        } else if value.len() == 3 {
            Ok(Colour::solid(value[0], value[1], value[2]))
        } else {
            Ok(Colour::new(value[0], value[1], value[2], value[3]))
        }
    }
}
impl TryFrom<&[u8]> for Colour {
    type Error = String;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() > 4 {
            Err("There are too many elements.".to_string())
        } else if value.len() < 3 {
            Err("There are not enough elements.".to_string())
        } else if value.len() == 3 {
            Ok(Colour::from_u8(value[0], value[1], value[2]))
        } else {
            Ok(Colour::from_u8_rgba(value[0], value[1], value[2], value[3]))
        }
    }
}

impl TryFrom<Vec<u8>> for Colour {
    type Error = String;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() > 4 {
            Err("There are too many elements.".to_string())
        } else if value.len() < 3 {
            Err("There are not enough elements.".to_string())
        } else if value.len() == 3 {
            Ok(Colour::from_u8(value[0], value[1], value[2]))
        } else {
            Ok(Colour::from_u8_rgba(value[0], value[1], value[2], value[3]))
        }
    }
}

impl Into<[f64; 4]> for Colour {
    fn into(self) -> [f64; 4] {
        [self.r, self.g, self.b, self.a]
    }
}
impl Into<(f64, f64, f64, f64)> for Colour {
    fn into(self) -> (f64, f64, f64, f64) {
        (self.r, self.g, self.b, self.a)
    }
}
impl Into<Vec<f64>> for Colour {
    fn into(self) -> Vec<f64> {
        Vec::from([self.r, self.g, self.b, self.a])
    }
}
impl Into<[u8; 4]> for Colour {
    fn into(self) -> [u8; 4] {
        let (r, g, b, a) = self.as_u8_rgba();
        [r, g, b, a]
    }
}
impl Into<(u8, u8, u8, u8)> for Colour {
    fn into(self) -> (u8, u8, u8, u8) {
        self.as_u8_rgba()
    }
}
impl Into<Vec<u8>> for Colour {
    fn into(self) -> Vec<u8> {
        let (r, g, b, a) = self.as_u8_rgba();
        Vec::from([r, g, b, a])
    }
}
#[cfg(feature = "ratatui")]
impl From<ratatui::style::Color> for Colour {
    fn from(colour: ratatui::style::Color) -> Colour {
        match colour {
            ratatui::style::Color::Black => Colour::from_u8(0, 0, 0),
            ratatui::style::Color::Red => Colour::from_u8(255, 0, 0),
            ratatui::style::Color::Green => Colour::from_u8(0, 255, 0),
            ratatui::style::Color::Yellow => Colour::from_u8(255, 255, 0),
            ratatui::style::Color::Blue => Colour::from_u8(0, 0, 255),
            ratatui::style::Color::Magenta => Colour::from_u8(255, 0, 255),
            ratatui::style::Color::Cyan => Colour::from_u8(0, 255, 255),
            ratatui::style::Color::Gray => Colour::from_u8(169, 169, 169),
            ratatui::style::Color::DarkGray => Colour::from_u8(128, 128, 128),
            ratatui::style::Color::LightRed => Colour::from_u8(255, 128, 128),
            ratatui::style::Color::LightGreen => Colour::from_u8(128, 255, 128),
            ratatui::style::Color::LightYellow => Colour::from_u8(255, 255, 128),
            ratatui::style::Color::LightBlue => Colour::from_u8(128, 128, 255),
            ratatui::style::Color::LightMagenta => Colour::from_u8(255, 128, 255),
            ratatui::style::Color::LightCyan => Colour::from_u8(128, 255, 255),
            ratatui::style::Color::White => Colour::from_u8(255, 255, 255),
            ratatui::style::Color::Rgb(r, g, b) => Colour::from_u8(r, g, b),
            ratatui::style::Color::Indexed(index) => {
                if index <= 6 {
                    Colour::from_u8(
                        (index & 0b100) * 0b11111111,
                        (index & 0b010) * 0b11111111,
                        (index & 0b001) * 0b11111111,
                    )
                } else if index == 7 {
                    Colour::from_u8(169, 169, 169)
                } else if index <= 15 {
                    Colour::from_u8(
                        (index & 0b100) * 0b01111111 + 0b10000000,
                        (index & 0b010) * 0b01111111 + 0b10000000,
                        (index & 0b001) * 0b01111111 + 0b10000000,
                    )
                } else if index < 232 {
                    // 6x6x6 color cube
                    let index = index - 16;
                    let r = (index / 36) * 51;
                    let g = ((index % 36) / 6) * 51;
                    let b = (index % 6) * 51;
                    return Colour::from_u8(r, g, b);
                } else {
                    // Grayscale ramp (232-255)
                    let gray = 8 + (index - 232) * 10;
                    return Colour::from_u8(gray, gray, gray);
                }
            }
            _ => Colour::from_u8(0, 0, 0), // Default case for unknown colors
        }
    }
}

#[cfg(feature = "ratatui")]
impl Into<ratatui::style::Color> for Colour {
    fn into(self) -> ratatui::style::Color {
        let (r, g, b) = self.as_u8();
        ratatui::style::Color::Rgb(r, g, b)
    }
}

#[cfg(feature = "ratatui")]
impl Into<ratatui::style::Color> for &Colour {
    fn into(self) -> ratatui::style::Color {
        let (r, g, b) = self.as_u8();
        ratatui::style::Color::Rgb(r, g, b)
    }
}

#[cfg(feature = "nalgebra")]
impl From<Vector4<f64>> for Colour {
    fn from(value: Vector4<f64>) -> Colour {
        Colour::new(value.x, value.y, value.z, value.w)
    }
}

#[cfg(feature = "nalgebra")]
impl Into<Vector4<f64>> for Colour {
    fn into(self) -> Vector4<f64> {
        Vector4::new(self.r, self.g, self.b, self.a)
    }
}

#[cfg(test)]
mod test {
    use approx::assert_relative_eq;

    use crate::Colour;

    #[test]
    pub fn ops_test() {
        let a = Colour::red(1.0);
        let b = Colour::grey(0.5);
        assert_relative_eq!(a * b, Colour::red(0.5));
        assert_relative_eq!(a + b, Colour::solid(1.5, 0.5, 0.5));
        assert_relative_eq!(a - b, Colour::solid(0.5, -0.5, -0.5));
        assert_relative_eq!(2f64 * b, Colour::grey(1.0));
        let c = Colour::grey(1.0);
        let d = Colour::transparent();
        assert_relative_eq!(d - c, Colour::grey(-1.0).with_alpha(0.0));
    }
}
