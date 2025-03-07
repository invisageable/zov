//! # The Hectic Curve.
//!
//! An algebraic curve of degree one hundred.
//!
//! #### Formula.
//!
//! `p^100`

use crate::easing::Curve;

/// ### The [`InHectic`] Easing Function.
///
/// #### Examples.
///
/// ```
/// use eazing::Curve;
/// use eazing::polynomial::hectic::InHectic;
///
/// let p = InHectic.y(1.0);
/// ```
#[derive(Debug)]
pub struct InHectic;

impl Curve for InHectic {
  #[inline]
  fn y(&self, p: f32) -> f32 {
    p.powi(100)
  }
}

#[test]
fn test_in_hectic() {
  let p = InHectic.y(1.0);

  assert_eq!(p, 1.0);
}

/// ### The [`OutHectic`] Easing Function.
///
/// #### Examples.
///
/// ```
/// use eazing::Curve;
/// use eazing::polynomial::hectic::OutHectic;
///
/// let p = OutHectic.y(1.0);
/// ```
#[derive(Debug)]
pub struct OutHectic;

impl Curve for OutHectic {
  #[inline]
  fn y(&self, p: f32) -> f32 {
    let m = p - 1.0;

    1.0 - m.powi(100)
  }
}

#[test]
fn test_out_hectic() {
  let p = OutHectic.y(1.0);

  assert_eq!(p, 1.0);
}

/// ### The [`InOutHectic`] Easing Function.
///
/// #### Examples.
///
/// ```
/// use eazing::Curve;
/// use eazing::polynomial::hectic::InOutHectic;
///
/// let p = InOutHectic.y(1.0);
/// ```
#[derive(Debug)]
pub struct InOutHectic;

impl Curve for InOutHectic {
  #[inline]
  fn y(&self, p: f32) -> f32 {
    let m = p - 1.0;
    let t = p * 2.0;

    if t < 1.0 {
      return p.powi(100) * 2.0;
    }

    1.0 - m.powi(100) * 2.0
  }
}

#[test]
fn test_in_out_hectic() {
  let p = InOutHectic.y(1.0);

  assert_eq!(p, 1.0);
}
