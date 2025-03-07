//! The Polynomial Cubic Interpolation.

use crate::easing::Curve;

use libm::{asinf, sinf};

/// The [`InPoCubic`] Curve.
///
/// ```
/// use eazing::Curve;
/// use eazing::interpolation::smoothstep::polynomial::po_cubic::InPoCubic;
///
/// let p = InPoCubic.y(1.0);
/// ```
#[derive(Debug)]
pub struct InPoCubic;

impl Curve for InPoCubic {
  #[inline]
  fn y(&self, p: f32) -> f32 {
    p * p * (3.0 - 2.0 * p)
  }
}

#[test]
fn test_in_po_cubic() {
  let p = InPoCubic.y(0.8);

  assert_eq!(p, 0.90450853);
}

/// The [`OutPoCubic`] Curve.
///
/// ```
/// use eazing::Curve;
/// use eazing::interpolation::smoothstep::polynomial::po_cubic::OutPoCubic;
///
/// let p = OutPoCubic.y(1.0);
/// ```
#[derive(Debug)]
pub struct OutPoCubic;

impl Curve for OutPoCubic {
  #[inline]
  fn y(&self, p: f32) -> f32 {
    0.5 - sinf(asinf(1.0 - 2.0 * p) / 3.0)
  }
}

#[test]
fn test_out_po_cubic() {
  let p = OutPoCubic.y(0.8);

  assert_eq!(p, 0.90450853);
}
