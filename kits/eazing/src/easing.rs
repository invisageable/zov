pub mod polynomial;

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
