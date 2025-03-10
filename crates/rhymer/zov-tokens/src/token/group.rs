//! The french group.
//!
//! @see — https://fr.wikipedia.org/wiki/Ponctuation.

/// The representation of a group.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Group {
  /// An open parenthesis — `(`.
  ParenOpen,
  /// A close parenthesis — `)`.
  ParenClose,
  /// An open bracket — `[`.
  BracketOpen,
  /// A close bracket — `]`.
  BracketClose,
}

impl std::fmt::Display for Group {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::ParenOpen => write!(f, "("),
      Self::ParenClose => write!(f, ")"),
      Self::BracketOpen => write!(f, "["),
      Self::BracketClose => write!(f, "]"),
    }
  }
}
