//! Each functions is grouped by context:
//!
//! #### easing functions.
//!
//! |               |                                   |
//! |:--------------|:----------------------------------|
//! | polynomial    | linear, quadratic, cubic, etc.    |
//! | trigonometric | sine, circle.                     |
//! | exponential   | expo2, expoe.                     |
//! | logarithmic   | log10.                            |
//! | root          | sqrt.                             |
//! | oscillatory   | elastic, bounce.                  |
//! | backtracking  | back.                             |
//!
//! #### notes.
//!
//! *Time is a normalized percentage of the elapsed time [0,1] i.e a range
//! between 0.0 and 1.0 (both inclusively).*

pub mod backtracking;
pub mod bezier;
pub mod exponential;
pub mod logarithmic;
pub mod oscillatory;
pub mod polynomial;
pub mod root;
pub mod trigonometric;

/// ### The [`Curve`] Parabola Interface.
///
/// A curve that has an axis of symmetry parallel to the `y-axis`.
pub trait Curve {
  /// Computes the `y-axis` of the curve from a progress value.
  ///     
  /// #### params.
  ///   
  /// |     |               |
  /// |:----|:--------------|
  /// | `p` | The progress. |
  ///   
  /// #### returns.
  ///
  /// `f32` — The progress of the `y-axis` value.
  ///   
  /// #### examples.
  ///   
  /// ```rust
  ///  use eazing::Curve;
  ///
  ///  struct Constant;
  ///  
  ///  impl Curve for Constant {
  ///    fn y(&self, p: f32) -> f32 {
  ///      -f32::INFINITY
  ///    }
  ///  }
  ///  
  ///  assert_eq!(Constant.y(0.0), -f32::INFINITY);
  /// ```
  fn y(&self, p: f32) -> f32;
}

/// ### The [`Ease`] Function Interface.
pub trait Ease {
  fn ease(p: f32, a: f32, b: f32) -> f32;
}

/// ### The [`Easing`] Function User Access.
///
/// Wraps all easing functions in one place.
#[derive(Default)]
pub enum Easing {
  // constant.
  None,
  // polynomial:linear.
  #[default]
  Linear,
  // polynomial:in.
  InQuadratic,
  InCubic,
  InQuartic,
  InQuintic,
  InSextic,
  InSeptic,
  InOctic,
  InNonic,
  InDecic,
  InHectic,
  // polynomial:out.
  OutQuadratic,
  OutCubic,
  OutQuartic,
  OutQuintic,
  OutSextic,
  OutSeptic,
  OutOctic,
  OutNonic,
  OutDecic,
  OutHectic,
  // polynomial:in-out.
  InOutQuadratic,
  InOutCubic,
  InOutQuartic,
  InOutQuintic,
  InOutSextic,
  InOutSeptic,
  InOutOctic,
  InOutNonic,
  InOutDecic,
  InOutHectic,
  // trigonometric:in.
  InSine,
  InCircle,
  // trigonometric:out.
  OutSine,
  OutCircle,
  // trigonometric:in-out.
  InOutSine,
  InOutCircle,
  // exponential:in.
  InExpo2,
  InExpoE,
  // exponential:out.
  OutExpo2,
  OutExpoE,
  // exponential:in-out.
  InOutExpo2,
  InOutExpoE,
  // logarithmic:in.
  InLog10,
  // logarithmic:out.
  OutLog10,
  // logarithmic:in-out.
  InOutLog10,
  // root:in.
  InSqrt,
  // root:out.
  OutSqrt,
  // root:in-out.
  InOutSqrt,
  // oscillatory:in.
  InElastic,
  InBounce,
  // oscillatory:out.
  OutElastic,
  OutBounce,
  // oscillatory:in-out.
  InOutElastic,
  InOutBounce,
  // backtracking:in.
  InBack,
  // backtracking:out.
  OutBack,
  // backtracking:in-out.
  InOutBack,
  // cubic bezier.
  CubicBezier(bezier::cubic_bezier::CubicBezier),
  /// smothstep.
  Smoothstep(crate::interpolation::smoothstep::Smoothstep),
  /// custom easing function.
  Custom(std::boxed::Box<dyn Curve>),
}

impl Curve for Easing {
  #[inline]
  fn y(&self, p: f32) -> f32 {
    match self {
      Self::None => power::none::None.y(p),
      Self::Linear => power::linear::Linear.y(p),
      Self::InQuadratic => power::quadratic::InQuadratic.y(p),
      Self::InCubic => power::cubic::InCubic.y(p),
      Self::InQuartic => power::quartic::InQuartic.y(p),
      Self::InQuintic => power::quintic::InQuintic.y(p),
      Self::InSextic => power::sextic::InSextic.y(p),
      Self::InSeptic => power::septic::InSeptic.y(p),
      Self::InOctic => power::octic::InOctic.y(p),
      Self::InNonic => power::nonic::InNonic.y(p),
      Self::InDecic => power::decic::InDecic.y(p),
      Self::InHectic => power::decic::InHectic.y(p),
      Self::OutQuadratic => power::quadratic::OutQuadratic.y(p),
      Self::OutCubic => power::cubic::OutCubic.y(p),
      Self::OutQuartic => power::quartic::OutQuartic.y(p),
      Self::OutQuintic => power::quintic::OutQuintic.y(p),
      Self::OutSextic => power::sextic::OutSextic.y(p),
      Self::OutSeptic => power::septic::OutSeptic.y(p),
      Self::OutOctic => power::octic::OutOctic.y(p),
      Self::OutNonic => power::nonic::OutNonic.y(p),
      Self::OutDecic => power::decic::OutDecic.y(p),
      Self::OutHectic => power::decic::OutHectic.y(p),
      Self::InOutQuadratic => power::quadratic::InOutQuadratic.y(p),
      Self::InOutCubic => power::cubic::InOutCubic.y(p),
      Self::InOutQuartic => power::quartic::InOutQuartic.y(p),
      Self::InOutQuintic => power::quintic::InOutQuintic.y(p),
      Self::InOutSextic => power::sextic::InOutSextic.y(p),
      Self::InOutSeptic => power::septic::InOutSeptic.y(p),
      Self::InOutOctic => power::octic::InOutOctic.y(p),
      Self::InOutNonic => power::nonic::InOutNonic.y(p),
      Self::InOutDecic => power::decic::InOutDecic.y(p),
      Self::InOutHectic => power::decic::InOutHectic.y(p),
      Self::InBack => standard::back::InBack.y(p),
      Self::InBounce => standard::bounce::InBounce.y(p),
      Self::InCircle => standard::circle::InCircle.y(p),
      Self::InElastic => standard::elastic::InElastic.y(p),
      Self::InExpo2 => standard::expo2::InExpo2.y(p),
      Self::InSine => standard::sine::InSine.y(p),
      Self::InOutBack => standard::back::InOutBack.y(p),
      Self::InOutBounce => standard::bounce::InOutBounce.y(p),
      Self::InOutCircle => standard::circle::InOutCircle.y(p),
      Self::InOutElastic => standard::elastic::InOutElastic.y(p),
      Self::InOutExpo2 => standard::expo2::InOutExpo2.y(p),
      Self::InOutSine => standard::sine::InOutSine.y(p),
      Self::OutBack => standard::back::OutBack.y(p),
      Self::OutBounce => standard::bounce::OutBounce.y(p),
      Self::OutCircle => standard::circle::OutCircle.y(p),
      Self::OutElastic => standard::elastic::OutElastic.y(p),
      Self::OutExpo2 => standard::expo2::OutExpo2.y(p),
      Self::OutSine => standard::sine::OutSine.y(p),
      Self::InExpoE => extras::expo_e::InExpoE.y(p),
      Self::InLog10 => extras::log10::InLog10.y(p),
      Self::InSqrt => extras::sqrt::InSqrt.y(p),
      Self::InOutExpoE => extras::expo_e::InOutExpoE.y(p),
      Self::InOutLog10 => extras::log10::InOutLog10.y(p),
      Self::InOutSqrt => extras::sqrt::InOutSqrt.y(p),
      Self::OutExpoE => extras::expo_e::InExpoE.y(p),
      Self::OutLog10 => extras::log10::OutLog10.y(p),
      Self::OutSqrt => extras::sqrt::OutSqrt.y(p),
      Self::CubicBezier(bezier) => bezier.y(p),
      Self::Smoothstep(smoothstep) => smoothstep.y(p),
      // shoud stay at the last place. New ones must be place above it.
      Self::Custom(curve) => curve.y(p),
    }
  }
}

/// ### The Ease Function.
///
/// Calculates the progress value.
///
/// #### notes.
///
/// The units of `start` and `end` values can be whatever you whish (px, ms,
/// etc.). But both should have been specified with the same consistent units.
/// Same for the `duration` and `elapsed` time.
#[inline]
pub fn ease(
  easing_fn: std::boxed::Box<dyn Curve>,
  elapsed: f32,
  start: f32,
  end: f32,
) -> f32 {
  start + (end - start) * easing_fn.y(time)
}
