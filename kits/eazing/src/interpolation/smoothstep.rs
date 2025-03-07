pub mod polynomial;
pub mod standard;
pub mod trigonometric;

/// The [`Smoothstep`] Interpolation.
#[derive(Debug)]
pub enum Smoothstep {
  /// smoothstep:in.
  InSmooth,
  InSmoother,
  /// smoothstep:out.
  OutSmooth,
  OutSmoother,
  // polynomial:in.
  InPoCubic,
  InPoQuartic,
  InPoQuintic,
  // polynomial:out.
  OutPoCubic,
  OutPoQuartic,
  OutPoQuintic,
  /// trigo:in.
  InTrigo,
  /// trigo:out.
  OutTrigo,
}

impl Curve for Smoothstep {
  #[inline]
  fn y(&self, p: f32) -> f32 {
    match self {
      Self::InSmooth => standard::smoothstep::InSmooth.y(p),
      Self::InSmoother => standard::smootherstep::InSmoother.y(p),
      Self::OutSmooth => standard::smoothstep::OutSmooth.y(p),
      Self::OutSmoother => standard::smootherstep::OutSmoother.y(p),
      Self::InPoCubic => polynomial::po_cubic::InPoCubic.y(p),
      Self::InPoQuartic => polynomial::po_quartic::InPoQuartic.y(p),
      Self::InPoQuintic => polynomial::po_quartic::InPoQuintic.y(p),
      Self::OutPoCubic => polynomial::po_cubic::OutPoCubic.y(p),
      Self::OutPoQuartic => polynomial::po_quartic::OutPoQuartic.y(p),
      Self::OutPoQuintic => polynomial::po_quartic::OutPoQuintic.y(p),
      Self::InTrigo => trigo::InTrigo.y(p),
      Self::OutTrigo => trigo::OutTrigo.y(p),
    }
  }
}
