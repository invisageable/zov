//! # The Square Root Curve.
//!
//! An algebric curve of degree 1/2.

use crate::easing::Curve;

use libm::sqrtf;

/// ### The [`InSqrt`] Easing Function.
///
/// #### examples.
///
/// ```rust
/// use eazing::Curve;
/// use eazing::root::sqrt::InSqrt;
///
/// let p = InSqrt.y(0.25);
/// ```
#[derive(Debug)]
pub struct InSqrt;

impl Curve for InSqrt {
  #[inline]
  fn y(&self, p: f32) -> f32 {
    1.0 - OutSqrt.y(1.0 - p)
  }
}

/// ### The [`InOutSqrt`] Easing Function.
///
/// #### examples.
///
/// ```rust
/// use eazing::Curve;
/// use eazing::root::sqrt::InOutSqrt;
///
/// let p = InOutSqrt.y(0.25);
/// ```
#[derive(Debug)]
pub struct InOutSqrt;

impl Curve for InOutSqrt {
  #[inline]
  fn y(&self, p: f32) -> f32 {
    let t = p * 2.0;

    if t < 1.0 {
      return 0.5 - 0.5 * OutSqrt.y(1.0 - t);
    }

    0.5 + 0.5 * OutSqrt.y(t - 1.0)
  }
}

/// ### The [`OutSqrt`] Easing Function.
///
/// #### examples.
///
/// ```rust
/// use eazing::Curve;
/// use eazing::root::sqrt::OutSqrt;
///
/// let p = OutSqrt.y(0.25);
/// ```
#[derive(Debug)]
pub struct OutSqrt;

impl Curve for OutSqrt {
  #[inline]
  fn y(&self, p: f32) -> f32 {
    sqrtf(p)
  }
}
