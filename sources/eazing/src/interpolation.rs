pub mod linear;
pub mod piecewize;
pub mod polynomial;
pub mod rational;
pub mod trigonometric;

use crate::Curve;

/// ### The [`Interpolation`] Function User Access.
#[derive(Debug)]
pub enum Interpolation {
  // constant.
  None,
  Lerp,
  // polynomial:in.
  InCubic,
  InQuartic,
  InQuintic,
  InSmooth,
  InSmoother,
  // polynomial:out.
  OutCubic,
  OutQuartic,
  OutQuintic,
  OutSmooth,
  OutSmoother,
  // rational:in.
  InRationalCubic,
  InRationalQuadratic,
  // rational:in.
  OutRationalCubic,
  OutRationalQuadratic,
  // piecewize:in.
  InPiecewizePolynomial,
  InPiecewizeQuadratic,
  // piecewize:in.
  OutPiecewizePolynomial,
  OutPiecewizeQuadratic,
  /// trigonometric:in.
  InSinusoidal,
  /// trigonometric:out.
  OutSinusoidal,
}

impl Curve for Interpolation {
  #[inline]
  fn y(&self, p: f32) -> f32 {
    match self {
      Self::None => todo!(),
      Self::Lerp => todo!(),
      Self::InCubic => polynomial::cubic::InCubic.y(p),
      Self::InQuartic => polynomial::quartic::InQuartic.y(p),
      Self::InQuintic => polynomial::quintic::InQuintic.y(p),
      Self::InSmooth => polynomial::smoothstep::InSmooth.y(p),
      Self::InSmoother => polynomial::smootherstep::InSmoother.y(p),
      Self::OutCubic => polynomial::cubic::OutCubic.y(p),
      Self::OutQuartic => polynomial::quartic::OutQuartic.y(p),
      Self::OutQuintic => polynomial::quintic::OutQuintic.y(p),
      Self::OutSmooth => polynomial::smoothstep::OutSmooth.y(p),
      Self::OutSmoother => polynomial::smootherstep::OutSmoother.y(p),
      Self::InRationalCubic => rational::cubic::InRationalCubic.y(p),
      Self::InRationalQuadratic => {
        rational::quadratic::InRationalQuadratic.y(p)
      }
      Self::OutRationalCubic => rational::cubic::OutRationalCubic.y(p),
      Self::OutRationalQuadratic => {
        rational::quadratic::OutRationalQuadratic.y(p)
      }

      Self::InPiecewizePolynomial => {
        piecewize::polynomial::InPiecewizePolynomial.y(p)
      }
      Self::InPiecewizeQuadratic => {
        piecewize::quadratic::InPiecewizeQuadratic.y(p)
      }
      Self::OutPiecewizePolynomial => {
        piecewize::polynomial::OutPiecewizePolynomial.y(p)
      }
      Self::OutPiecewizeQuadratic => {
        piecewize::quadratic::OutPiecewizeQuadratic.y(p)
      }
      Self::InSinusoidal => trigonometric::sinusoidal::InSinusoidal.y(p),
      Self::OutSinusoidal => trigonometric::sinusoidal::OutSinusoidal.y(p),
    }
  }
}
