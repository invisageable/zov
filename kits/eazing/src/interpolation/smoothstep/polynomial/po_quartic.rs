//! The Polynomial Quartic Interpolation.

use crate::easing::Curve;

use libm::sqrtf;

/// The [`InPoQuartic`] Curve.
#[derive(Debug)]
pub struct InPoQuartic;

impl Curve for InPoQuartic {
  #[inline]
  fn y(&self, p: f32) -> f32 {
    p * p * (2.0 - p * p)
  }
}

#[test]
fn test_in_po_quartic() {
  let p = InPoQuartic.y(1.0);

  assert_eq!(p, 1.0);
}

/// The [`OutPoQuartic`] Curve.
#[derive(Debug)]
pub struct OutPoQuartic;

impl Curve for OutPoQuartic {
  #[inline]
  fn y(&self, p: f32) -> f32 {
    sqrtf(1.0 - sqrtf(1.0 - p))
  }
}

#[test]
fn test_out_po_quartic() {
  let p = OutPoQuartic.y(1.0);

  assert_eq!(p, 1.0);
}
