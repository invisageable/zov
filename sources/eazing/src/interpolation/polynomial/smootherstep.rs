//! The Smootherstep Interpolating Polynomial Curve.

use crate::easing::Curve;

/// The [`InSmoother`] Curve.
///
/// #### examples.
///
/// ```
/// use eazing::Curve;
/// use eazing::interpolation::polynomial::smootherstep::InSmoother;
///
/// let p = InSmoother.y(1.0);
/// ```
#[derive(Debug)]
pub struct InSmoother;

impl Curve for InSmoother {
  #[inline]
  fn y(&self, _p: f32) -> f32 {
    unimplemented!()
  }
}

#[test]
fn test_in_smoother() {
  let p = InSmoother.y(0.8);

  assert_eq!(p, 0.90450853);
}

/// The [`OutSmoother`] Curve.
///
/// #### examples.
///
/// ```
/// use eazing::Curve;
/// use eazing::interpolation::polynomial::smootherstep::OutSmoother;
///
/// let p = OutSmoother.y(1.0);
/// ```
#[derive(Debug)]
pub struct OutSmoother;

impl Curve for OutSmoother {
  #[inline]
  fn y(&self, _p: f32) -> f32 {
    unimplemented!()
  }
}

#[test]
fn test_out_smoother() {
  let p = OutSmoother.y(0.8);

  assert_eq!(p, 0.90450853);
}

/// The Non-linear Interpolation.
///
/// Interpolates smoothly between `min` and `max`. It will accelerated from the
/// start and deccelerated toward the end with a cubic easing.
///
/// #### params.
///
/// |      |                            |
/// |:-----|:---------------------------|
/// | `p`  | The progress.              |
/// | `x0` | The `min` start value.     |
/// | `x1` | The `max` end value.       |
///
/// #### returns.
///
/// `f32` — The interpolated result between the two float values.
///
/// #### examples.
///
/// ```
/// use eazing::interpolation::polynomial::smootherstep::smootherstep;
///
/// let p = smootherstep(0.25, 0.0, 1.0);
///
/// assert_eq!(p, 0.103515625);
/// ```
///
/// #### notes.
///
/// The formula was suggested by Ken Perlin. For more information about the
/// formula go to the [wiki](<https://en.wikipedia.org/wiki/Smoothstep>)
#[inline]
pub fn smootherstep(p: f32, x0: f32, x1: f32) -> f32 {
  let mut p = (p - x0) / (x1 - x0);

  p = p.clamp(0.0, 1.0);
  p * p * p * (p * (6.0 * p - 15.0) + 10.0)
}
