//! # The Cubic Bézier Curve.
//!
//! More information [here](https://drafts.csswg.org/css-easing/#cubic-bezier-easing-functions).

use crate::easing::Curve;

use lyon_geom::{CubicBezierSegment, Point};

/// The [`CubicBezier`] Easing Function.
///
/// #### notes.
///
/// See also [`crate::easing::Easing`].
#[derive(Clone, Copy, Debug)]
pub struct CubicBezier {
  segment: CubicBezierSegment<f32>,
}

impl CubicBezier {
  /// Creates a [`CubicBezier`] curve from two control points.
  ///
  /// `p0` and `p3` are fixed to (0,0) and (1,1). When `p1` and `p2` are
  /// restricted to range [0,1]. The output progress values are `y-axis`.
  ///
  /// #### params.
  ///
  /// |       |                                              |
  /// |:------|----------------------------------------------|
  /// | `p1x` | The position of the `x-axis` of `p1` control |
  /// | `p1y` | The position of the `y-axis` of `p1` control |
  /// | `p2x` | The position of the `x-axis` of `p2` control |
  /// | `p2y` | The position of the `y-axis` of `p2` control |
  ///
  /// #### examples.
  ///
  /// ```rust
  /// use eazing::Curve;
  /// use eazing::bezier::cubic::CubicBezier;
  ///
  /// let cubic_bezier = CubicBezier::curve(0.25, 0.1, 0.25, 1.0);
  ///
  /// assert_eq!(cubic_bezier.y(0.0), 0.0);
  /// assert_eq!(cubic_bezier.y(0.5), 0.5375);
  /// assert_eq!(cubic_bezier.y(1.0), 1.0);
  /// ```
  #[inline]
  pub fn curve(p1x: f32, p1y: f32, p2x: f32, p2y: f32) -> Self {
    Self {
      segment: CubicBezierSegment {
        from: Point::origin(),
        ctrl1: Point::new(p1x, p1y),
        ctrl2: Point::new(p2x, p2y),
        to: Point::new(1.0, 1.0),
      },
    }
  }

  /// Creates a [`CubicBezier::curve`] based on CSS `ease`.
  #[inline]
  pub fn ease() -> Self {
    Self::curve(0.25, 0.1, 0.25, 1.0)
  }

  /// Creates a [`CubicBezier::curve`] based on CSS `ease-in`.
  #[inline]
  pub fn in_ease() -> Self {
    Self::curve(0.42, 0.0, 1.0, 1.0)
  }

  /// Creates a [`CubicBezier::curve`] based on CSS `ease-in-out`.
  #[inline]
  pub fn in_out_ease() -> Self {
    Self::curve(0.0, 0.0, 0.58, 1.0)
  }

  /// Creates a [`CubicBezier::curve`] based on CSS `ease-out`.
  #[inline]
  pub fn out_ease() -> Self {
    Self::curve(0.42, 0.0, 0.58, 1.0)
  }
}

impl Curve for CubicBezier {
  #[inline]
  fn y(&self, p: f32) -> f32 {
    let p = p.clamp(0.0, 1.0);

    self.segment.y(p)
  }
}

#[test]
fn test_cubic_bezier_segment_y() {
  let cubic_bezier = CubicBezier::curve(0.17, 0.67, 0.83, 0.67);

  assert_eq!(cubic_bezier.y(0.0), 0.0);
  assert_eq!(cubic_bezier.y(0.5), 0.6275);
  assert_eq!(cubic_bezier.y(1.0), 1.0);
}
