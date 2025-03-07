//! # The Decic Curve.
//!
//! An algebric curve of degree ten.
//!
//! #### formula.
//!
//! `p^10`

use crate::easing::Curve;

/// ### The [`InDecic`] Easing Function.
///
/// #### examples.
///
/// ```
/// use eazing::Curve;
/// use eazing::polynomial::decic::InDecic;
///
/// let p = InDecic.y(1.0);
/// ```
#[derive(Debug)]
pub struct InDecic;

impl Curve for InDecic {
  #[inline]
  fn y(&self, p: f32) -> f32 {
    p * p * p * p * p * p * p * p * p * p
  }
}

#[test]
fn test_in_decic() {
  let p = InDecic.y(1.0);

  assert_eq!(p, 1.0);
}

/// ### The [`OutDecic`] Easing Function.
///
/// #### examples.
///
/// ```
/// use eazing::Curve;
/// use eazing::polynomial::decic::OutDecic;
///
/// let p = OutDecic.y(1.0);
/// ```
#[derive(Debug)]
pub struct OutDecic;

impl Curve for OutDecic {
  #[inline]
  fn y(&self, p: f32) -> f32 {
    let m = p - 1.0;

    1.0 - m * m * m * m * m * m * m * m * m * m
  }
}

fn test_out_decic() {
  let p = OutDecic.y(1.0);

  assert_eq!(p, 1.0);
}

/// ### The [`InOutDecic`] Easing Function.
///
/// #### examples.
///
/// ```
/// use eazing::Curve;
/// use eazing::polynomial::decic::InOutDecic;
///
/// let p = InOutDecic.y(1.0);
/// ```
#[derive(Debug)]
pub struct InOutDecic;

impl Curve for InOutDecic {
  #[inline]
  fn y(&self, p: f32) -> f32 {
    let m = p - 1.0;
    let t = p * 2.0;

    if t < 1.0 {
      return p * t * t * t * t * t * t * t * t;
    }

    1.0 - m * m * m * m * m * m * m * m * m * m * 512.0
  }
}

fn test_in_out_decic() {
  let p = InOutDecic.y(1.0);

  assert_eq!(p, 1.0);
}
