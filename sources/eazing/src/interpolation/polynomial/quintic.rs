//! The Polynomial Quintic Interpolation.

use crate::easing::Curve;

/// The [`InQuintic`] Curve.
#[derive(Debug)]
pub struct InQuintic;

impl Curve for InQuintic {
  #[inline]
  fn y(&self, p: f32) -> f32 {
    p * p * p * (p * (p * 6.0 - 15.0) + 10.0)
  }
}

#[test]
fn test_in_quintic() {
  let p = InQuintic.y(1.0);

  assert_eq!(p, 1.0);
}

/// The [`OutQuintic`] Curve.
#[derive(Debug)]
pub struct OutQuintic;

impl Curve for OutQuintic {
  #[inline]
  fn y(&self, _p: f32) -> f32 {
    unimplemented!()
  }
}

#[test]
fn test_out_quintic() {
  let _p = OutQuintic.y(1.0);

  // assert_eq!(p, 1.0);
}
