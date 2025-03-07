use crate::easing::Curve;

use libm::{asinf, sinf};

/// The [`InSmooth`] Curve.
///
/// #### examples.
///
/// ```
/// use eazing::Curve;
/// use eazing::interpolation::smoothstep::InSmooth;
///
/// let p = InSmooth.y(1.0);
/// ```
#[derive(Debug)]
pub struct InSmooth;

impl Curve for InSmooth {
  #[inline]
  fn y(&self, p: f32) -> f32 {
    let mut p = p;

    p = p.clamp(0.0, 1.0);
    p * p * (3.0 - 2.0 * p)
  }
}

#[test]
fn test_in_smooth() {
  let p = InSmooth.y(0.8);

  assert_eq!(p, 0.90450853);
}

/// The [`OutSmooth`] Curve.
///
/// #### examples.
///
/// ```
/// use eazing::Curve;
/// use eazing::interpolation::smoothstep::OutSmooth;
///
/// let p = OutSmooth.y(1.0);
/// ```
#[derive(Debug)]
pub struct OutSmooth;

impl Curve for OutSmooth {
  #[inline]
  fn y(&self, p: f32) -> f32 {
    0.5 - sinf(asinf(1.0 - 2.0 * p) / 3.0)
  }
}

#[test]
fn test_out_smooth() {
  let p = OutSmooth.y(0.8);

  assert_eq!(p, 0.90450853);
}
