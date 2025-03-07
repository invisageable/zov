//! The Polynomial Quintic Interpolation.

use crate::easing::Curve;

/// The [`InPoQuintic`] Curve.
#[derive(Debug)]
pub struct InPoQuintic;

impl Curve for InPoQuintic {
  #[inline]
  fn y(&self, p: f32) -> f32 {
    p * p * p * (p * (p * 6.0 - 15.0) + 10.0)
  }
}

#[test]
fn test_in_po_quintic() {
  let p = InPoQuintic.y(1.0);

  assert_eq!(p, 1.0);
}
