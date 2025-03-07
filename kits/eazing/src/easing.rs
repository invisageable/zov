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
  CubicBezier(bezier::Bezier),
  /// smothstep.
  Smoothstep(crate::interpolation::smoothstep::Smoothstep),
  /// custom easing function.
  Custom(std::boxed::Box<dyn Curve>),
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
