//! The french quote.
//!
//! @see — https://fr.wikipedia.org/wiki/Ponctuation.

/// The representation of a quote group.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quote {
  /// A double quote — `"`.
  Double,
  /// An open french quote — `«`.
  FrenchOpen,
  /// A close french quote — `»`.
  FrenchClose,
  /// An open chevron quote — `‹`.
  ChevronOpen,
  /// A close chevron quote — `›`.
  ChevronClose,
}

impl std::fmt::Display for Quote {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Double => write!(f, "\""),
      Self::FrenchOpen => write!(f, "«"),
      Self::FrenchClose => write!(f, "»"),
      Self::ChevronOpen => write!(f, "‹"),
      Self::ChevronClose => write!(f, "›"),
    }
  }
}
