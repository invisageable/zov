//! # The Cubic Curve.
//!
//! An algebric curve of degree three.
//!
//! #### formula.
//!
//! `p^3`

use crate::easing::Curve;

/// ### The [`InCubic`] Easing Function.
///
/// #### examples.
///
/// ```
/// use eazing::Curve;
/// use eazing::polynomial::cubic::InCubic;
///
/// let p = InCubic.y(1.0);
/// ```
/// `f(t) = 2.0^(10.0 * (t - 1.0))`
#[derive(Debug)]
pub struct InCubic;

impl Curve for InCubic {
  #[inline]
  fn y(&self, p: f32) -> f32 {
    p * p * p
  }
}

#[test]
fn test_in_cubic() {
  let p = InCubic.y(1.0);

  assert_eq!(p, 1.0);
}

/// ### The [`OutCubic`] Easing Function.
///
/// #### examples.
///
/// ```
/// use eazing::Curve;
/// use eazing::polynomial::cubic::OutCubic;
///
/// let p = OutCubic.y(1.0);
/// ```
#[derive(Debug)]
pub struct OutCubic;

impl Curve for OutCubic {
  #[inline]
  fn y(&self, p: f32) -> f32 {
    let m = p - 1.0;

    1.0 + m * m * m
  }
}

#[test]
fn test_out_cubic() {
  let p = OutCubic.y(1.0);

  assert_eq!(p, 1.0);
}

/// ### The [`InOutCubic`] Easing Function.
///
/// #### examples.
///
/// ```
/// use eazing::Curve;
/// use eazing::polynomial::cubic::InOutCubic;
///
/// let p = InOutCubic.y(1.0);
/// ```
#[derive(Debug)]
pub struct InOutCubic;

impl Curve for InOutCubic {
  #[inline]
  fn y(&self, p: f32) -> f32 {
    let m = p - 1.0;
    let t = p * 2.0;

    if t < 1.0 {
      return p * t * t;
    }

    1.0 + m * m * m * 4.0
  }
}

#[test]
fn test_in_out_cubic() {
  let p = InOutCubic.y(1.0);

  assert_eq!(p, 1.0);
}
