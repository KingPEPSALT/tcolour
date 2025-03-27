use std::ops::Not;

use crate::colour::Colour;

pub type GradientStop = (f64, Colour);
pub struct Gradient(pub Vec<GradientStop>);

impl Gradient {
    /// Inserts (t: f64, colour: Colour) in the region that `t` resides
    /// if `t` exists, this will replace the colour.
    pub fn insert(&mut self, t: f64, colour: Colour) {
        if let Some(index) = self
            .0
            .iter()
            .enumerate()
            .skip_while(|(_, (v, _))| *v < t)
            .next()
            .map(|(i, (_, _))| i)
        {
            if self.0[index].0 == t {
                self.0[index].1 = colour;
            } else {
                self.0.insert(index, (t, colour));
            }
        } else {
            self.0.push((t, colour));
        };
    }

    /// Returns the two gradient stops that `t` resides between.
    ///
    /// Much like a quadratic solution, if `t` resides before the first
    /// gradient stop, that gradient stop will be returned twice
    /// and similarly with the last gradient stop. Interpolation between
    /// these, no matter the `t` value and it's validity, will always return
    /// the colour of that gradient stop.
    ///
    /// # Example
    ///
    /// ```
    /// use tcolour::{Gradient, Colour};
    /// let gradient = Gradient(vec![
    ///     (0.5, Colour::solid(1.0, 0.0, 0.0)),
    ///     (0.7, Colour::solid(0.0, 1.0, 0.0)),
    ///     (0.8, Colour::solid(0.0, 0.0, 1.0)),
    /// ]);
    ///
    /// assert_eq!(gradient.subgradient(0.1), ((0.5, Colour::red(1.0)), (0.5, Colour::red(1.0))));
    /// assert_eq!(gradient.subgradient(0.6), ((0.5, Colour::red(1.0)), (0.7, Colour::green(1.0))));
    /// assert_eq!(gradient.subgradient(0.9), ((0.8, Colour::blue(1.0)), (0.8, Colour::blue(1.0))));
    /// ```
    pub fn subgradient(&self, t: f64) -> (GradientStop, GradientStop) {
        self.0
            .iter()
            .scan(None, |prev, &curr| {
                let old = prev.take();
                *prev = Some(curr);
                if curr.0 > t {
                    Some(Some((old.unwrap_or(curr), curr)))
                } else {
                    Some(None)
                }
            })
            .find_map(|x| x)
            .or_else(|| self.0.last().map(|&last| (last, last)))
            .unwrap()
    }

    /// Gets a colour from the gradient using
    /// linear interpolation. Use `.interpolate()`
    /// for your own interpolation function and
    /// use `.pick()`` to just get the colour that
    /// `t` lands on. The alpha value is also
    /// interpolated.
    ///
    /// # Example
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use tcolour::{Gradient, Colour};
    /// let gradient = Gradient(vec![
    ///     (0.5, Colour::solid(1.0, 0.0, 0.0)),
    ///     (0.7, Colour::solid(0.0, 1.0, 0.0)),
    ///     (0.8, Colour::solid(0.0, 0.0, 1.0)),
    /// ]);
    ///
    /// assert_eq!(gradient.sample(0.6), Colour::solid(0.5, 0.5, 0.0));
    /// assert_relative_eq!(gradient.sample(0.65), Colour::solid(0.25, 0.75, 0.0));
    /// assert_eq!(gradient.sample(0.9), Colour::solid(0.0, 0.0, 1.0));
    /// ```
    pub fn sample(&self, t: f64) -> Colour {
        self.interpolate(t, |from, to, t| {
            (from + (to - from) * t).with_alpha(from.a + (to.a - from.a) * t)
        })
    }

    /// Gets a colour from the gradient by finding
    /// the region that contains `t` and then interpolating
    /// using the function that is given.
    ///
    /// `t` is normalised between `t_from` and `t_to` by
    /// `t = (t - t_from)/(t_to - t_from)` such that
    /// with `t = 0.7`, `t_from = 0.6` and `t_to = 0.8`
    /// we actually interpolate between `[0, 1]` with `t = 0.5`
    pub fn interpolate<F: FnOnce(Colour, Colour, f64) -> Colour>(
        &self,
        t: f64,
        interpolator: F,
    ) -> Colour {
        let ((t_from, from), (t_to, to)) = self.subgradient(t);
        let normalised_t = (t - t_from) / (t_to - t_from);
        interpolator(
            from,
            to,
            normalised_t.is_normal().not().then_some(1f64).unwrap_or(normalised_t)
        )
    }

    /// Select the lower bound colour, use `.select_upper()` for the upper
    /// bound colour
    pub fn select(&self, t: f64) -> Colour {
        self.subgradient(t).0.1
    }

    /// Select the upper bound colour, use `.select()` for the lower bound
    /// colour
    pub fn select_upper(&self, t: f64) -> Colour {
        self.subgradient(t).1.1
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::Gradient;
    use crate::Colour;

    #[test]
    pub fn subgradient_test() {
        let gradient = Gradient(vec![
            (0.5, Colour::solid(1.0, 0.0, 0.0)),
            (0.7, Colour::solid(0.0, 1.0, 0.0)),
            (0.8, Colour::solid(0.0, 0.0, 1.0)),
        ]);
        assert_eq!(
            gradient.subgradient(0.1),
            (
                (0.5, Colour::solid(1.0, 0.0, 0.0)),
                (0.5, Colour::solid(1.0, 0.0, 0.0))
            )
        );
        assert_eq!(
            gradient.subgradient(0.6),
            (
                (0.5, Colour::solid(1.0, 0.0, 0.0)),
                (0.7, Colour::solid(0.0, 1.0, 0.0))
            )
        );
        assert_eq!(
            gradient.subgradient(0.75),
            (
                (0.7, Colour::solid(0.0, 1.0, 0.0)),
                (0.8, Colour::solid(0.0, 0.0, 1.0))
            )
        );
        assert_eq!(
            gradient.subgradient(0.9),
            (
                (0.8, Colour::solid(0.0, 0.0, 1.0)),
                (0.8, Colour::solid(0.0, 0.0, 1.0))
            )
        );
    }

    #[test]
    pub fn insertion_test() {
        let mut gradient = Gradient(vec![
            (0.5, Colour::solid(1.0, 0.0, 0.0)),
            (0.7, Colour::solid(0.0, 1.0, 0.0)),
            (0.8, Colour::solid(0.0, 0.0, 1.0)),
        ]);
        gradient.insert(0.3, Colour::grey(0.5));
        assert_eq!(gradient.0.first().unwrap().1, Colour::grey(0.5));
        gradient.insert(0.9, Colour::grey(0.2));
        assert_eq!(gradient.0.last().unwrap().1, Colour::grey(0.2));
        gradient.insert(0.6, Colour::red(0.8));
        assert_eq!(gradient.0[2].1, Colour::red(0.8));
        gradient.insert(0.85, Colour::transparent());
        assert_eq!(gradient.0[5].1, Colour::transparent());
        assert_eq!(gradient.0[6].1, Colour::grey(0.2));
        gradient.insert(0.5, Colour::red(1.0).with_blue(1.0));
        assert_eq!(
            gradient.0,
            vec![
                (0.3, Colour::grey(0.5)),
                (0.5, Colour::red(1.0).with_blue(1.0)),
                (0.6, Colour::red(0.8)),
                (0.7, Colour::green(1.0)),
                (0.8, Colour::blue(1.0)),
                (0.85, Colour::transparent()),
                (0.9, Colour::grey(0.2))
            ]
        );
    }

    #[test]
    pub fn interpolation_test() {
        let gradient = Gradient(vec![
            (0.5, Colour::solid(1.0, 0.0, 0.0)),
            (0.7, Colour::solid(0.0, 1.0, 0.0)),
            (0.8, Colour::solid(0.0, 0.0, 1.0)),
        ]);

        let other_gradient = Gradient(vec![(0.4, Colour::grey(1.0)), (0.6, Colour::transparent())]);
        assert_eq!(other_gradient.sample(0.5), Colour::grey(0.5).with_alpha(0.5));
        assert_eq!(gradient.sample(0.6), Colour::solid(0.5, 0.5, 0.0));
        assert_relative_eq!(other_gradient.sample(0.8), Colour::transparent());
        assert_eq!(gradient.sample(0.1), Colour::red(1.0));
    }
}
