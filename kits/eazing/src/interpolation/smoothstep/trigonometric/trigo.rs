use crate::easing::Curve;

use libm::{acosf, cosf};

/// The [`InTrigo`] Curve.
#[derive(Debug)]
pub struct InTrigo;

impl Curve for InTrigo {
  #[inline]
  fn y(&self, p: f32) -> f32 {
    0.5 - 0.5 * cosf(core::f32::consts::PI * p)
  }
}

#[test]
fn test_in_trigo() {
  let p = InTrigo.y(0.8);

  assert_eq!(p, 0.90450853);
}

/// The [`OutTrigo`] Curve.
#[derive(Debug)]
pub struct OutTrigo;

impl Curve for OutTrigo {
  #[inline]
  fn y(&self, p: f32) -> f32 {
    acosf(1.0 - 2.0 * p) / core::f32::consts::PI
  }
}

#[test]
fn test_out_trigo() {
  let p = OutTrigo.y(1.0);

  assert_eq!(p, 0.99999994);
}
