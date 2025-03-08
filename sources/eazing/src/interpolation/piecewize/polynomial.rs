//! The Piecewize Polynomial Interpolation.

use crate::easing::Curve;

/// The [`InPiecewizePolynomial`] Curve.
#[derive(Debug)]
pub struct InPiecewizePolynomial;

impl Curve for InPiecewizePolynomial {
  #[inline]
  fn y(&self, _p: f32) -> f32 {
    unimplemented!()
  }
}

/// The [`OutPiecewizePolynomial`] Curve.
#[derive(Debug)]
pub struct OutPiecewizePolynomial;

impl Curve for OutPiecewizePolynomial {
  #[inline]
  fn y(&self, _p: f32) -> f32 {
    unimplemented!()
  }
}
