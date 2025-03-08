//! The Polynomial Quartic Interpolation.

use crate::easing::Curve;

use libm::sqrtf;

/// The [`InQuartic`] Curve.
#[derive(Debug)]
pub struct InQuartic;

impl Curve for InQuartic {
  #[inline]
  fn y(&self, p: f32) -> f32 {
    p * p * (2.0 - p * p)
  }
}

#[test]
fn test_in_quartic() {
  let p = InQuartic.y(1.0);

  assert_eq!(p, 1.0);
}

/// The [`OutQuartic`] Curve.
#[derive(Debug)]
pub struct OutQuartic;

impl Curve for OutQuartic {
  #[inline]
  fn y(&self, p: f32) -> f32 {
    sqrtf(1.0 - sqrtf(1.0 - p))
  }
}

#[test]
fn test_out_quartic() {
  let p = OutQuartic.y(1.0);

  assert_eq!(p, 1.0);
}
