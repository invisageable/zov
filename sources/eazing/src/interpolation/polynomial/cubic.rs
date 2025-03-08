//! The Polynomial Cubic Interpolation.

use crate::easing::Curve;

use libm::{asinf, sinf};

/// The [`InCubic`] Curve.
///
/// ```
/// use eazing::Curve;
/// use eazing::interpolation::polynomial::cubic::InCubic;
///
/// let p = InCubic.y(1.0);
/// ```
#[derive(Debug)]
pub struct InCubic;

impl Curve for InCubic {
  #[inline]
  fn y(&self, p: f32) -> f32 {
    p * p * (3.0 - 2.0 * p)
  }
}

#[test]
fn test_in_cubic() {
  let p = InCubic.y(0.8);

  assert_eq!(p, 0.90450853);
}

/// The [`OutCubic`] Curve.
///
/// ```
/// use eazing::Curve;
/// use eazing::interpolation::polynomial::cubic::OutCubic;
///
/// let p = OutCubic.y(1.0);
/// ```
#[derive(Debug)]
pub struct OutCubic;

impl Curve for OutCubic {
  #[inline]
  fn y(&self, p: f32) -> f32 {
    0.5 - sinf(asinf(1.0 - 2.0 * p) / 3.0)
  }
}

#[test]
fn test_out_cubic() {
  let p = OutCubic.y(0.8);

  assert_eq!(p, 0.90450853);
}
