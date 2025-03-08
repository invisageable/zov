use crate::easing::Curve;

use libm::{acosf, cosf};

/// The [`InSinusoidal`] Curve.
#[derive(Debug)]
pub struct InSinusoidal;

impl Curve for InSinusoidal {
  #[inline]
  fn y(&self, p: f32) -> f32 {
    0.5 - 0.5 * cosf(core::f32::consts::PI * p)
  }
}

#[test]
fn test_in_sinusoidal() {
  let p = InSinusoidal.y(0.8);

  assert_eq!(p, 0.90450853);
}

/// The [`OutSinusoidal`] Curve.
#[derive(Debug)]
pub struct OutSinusoidal;

impl Curve for OutSinusoidal {
  #[inline]
  fn y(&self, p: f32) -> f32 {
    acosf(1.0 - 2.0 * p) / core::f32::consts::PI
  }
}

#[test]
fn test_out_sinusoidal() {
  let p = OutSinusoidal.y(1.0);

  assert_eq!(p, 0.99999994);
}
