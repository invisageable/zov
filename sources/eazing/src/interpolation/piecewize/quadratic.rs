//! The Piecewize Quadratic Interpolation.

use crate::easing::Curve;

use libm::sqrtf;

/// The [`InPiecewizeQuadratic`] Curve.
#[derive(Debug)]
pub struct InPiecewizeQuadratic;

impl Curve for InPiecewizeQuadratic {
  #[inline]
  fn y(&self, p: f32) -> f32 {
    if p < 0.5 {
      return 2.0 * p * p;
    }

    2.0 * p * (2.0 - p) - 1.0
  }
}

#[test]
fn test_in_piecewize_quadratic() {
  let p = InPiecewizeQuadratic.y(1.0);

  assert_eq!(p, 1.0);
}

/// The [`OutPiecewizeQuadratic`] Curve.
#[derive(Debug)]
pub struct OutPiecewizeQuadratic;

impl Curve for OutPiecewizeQuadratic {
  #[inline]
  fn y(&self, p: f32) -> f32 {
    if p < 0.5 {
      return sqrtf(0.5 * p);
    }

    1.0 - sqrtf(0.5 - 0.5 * p)
  }
}

#[test]
fn test_out_piecewize_quadratic() {
  let p = OutPiecewizeQuadratic.y(1.0);

  assert_eq!(p, 1.0);
}
