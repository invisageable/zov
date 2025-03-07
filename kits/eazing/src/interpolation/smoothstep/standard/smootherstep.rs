use crate::easing::Curve;

/// The [`InSmoother`] Curve.
///
/// #### examples.
///
/// ```
/// use eazing::Curve;
/// use eazing::interpolation::InSmoother;
///
/// let p = InSmoother.y(1.0);
/// ```
#[derive(Debug)]
pub struct InSmoother;

impl Curve for InSmoother {
  #[inline]
  fn y(&self, _p: f32) -> f32 {
    unimplemented!()
  }
}

#[test]
fn test_in_smoother() {
  let p = InSmoother.y(0.8);

  assert_eq!(p, 0.90450853);
}

/// The [`OutSmoother`] Curve.
///
/// #### examples.
///
/// ```
/// use eazing::Curve;
/// use eazing::interpolation::smoothstep::OutSmoother;
///
/// let p = OutSmoother.y(1.0);
/// ```
#[derive(Debug)]
pub struct OutSmoother;

impl Curve for OutSmoother {
  #[inline]
  fn y(&self, _p: f32) -> f32 {
    unimplemented!()
  }
}

#[test]
fn test_out_smoother() {
  let p = OutSmoother.y(0.8);

  assert_eq!(p, 0.90450853);
}
