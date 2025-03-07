//! # The Linear Curve.
//!
//! An algebric curve of degree one.
//!
//! #### formula.
//!
//! `p`

use crate::easing::Curve;

/// ### The [`Linear`] Easing Function.
///
/// #### examples.
///
/// ```
/// ```
#[derive(Debug)]
pub struct Linear;

impl Curve for Linear {
  #[inline]
  fn y(&self, p: f32) -> f32 {
    p
  }
}

#[cfg(test)]
mod tests {
  use super::Linear;
  use crate::easing::Curve;

  #[test]
  fn test_linear() {
    let p = Linear.y(100.0);

    assert_eq!(p, 100.0);
  }
}
