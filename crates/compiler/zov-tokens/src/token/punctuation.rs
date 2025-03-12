//! The french punctuation.
//!
//! @see — https://fr.wikipedia.org/wiki/Ponctuation.

/// The representation of a punctuation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Punctuation {
  /// A dot punctuation — `.`.
  Dot,
  /// A semicolon punctuation — `;`.
  Semi,
  /// A comma punctuation — `,`.
  Comma,
  /// A question mark punctuation — `?`.
  Question,
  /// An exclamation punctuation — `!`.
  Exclamation,
  /// A dot dot dot punctuation — `...`.
  DotDotDot,
  /// A colon punctuation — `:`.
  Colon,
  /// A figure dash punctuation — `-`.
  FigureDash,
  /// A en dash punctuation — `—`.
  EnDash,
  /// An apostrophe punctuation — `'`.
  Apostrophe,
  /// A slash punctuation — `/`.
  Slash,
}

impl std::fmt::Display for Punctuation {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Dot => write!(f, "."),
      Self::Semi => write!(f, ";"),
      Self::Comma => write!(f, ","),
      Self::Question => write!(f, "?"),
      Self::Exclamation => write!(f, "!"),
      Self::DotDotDot => write!(f, "..."),
      Self::Colon => write!(f, ":"),
      Self::FigureDash => write!(f, "-"),
      Self::EnDash => write!(f, "—"),
      Self::Apostrophe => write!(f, "'"),
      Self::Slash => write!(f, "/"),
    }
  }
}

impl From<&str> for Punctuation {
  fn from(punc: &str) -> Self {
    match punc {
      "." => Self::Dot,
      ";" => Self::Semi,
      "," => Self::Comma,
      "?" => Self::Question,
      "!" => Self::Exclamation,
      "..." => Self::DotDotDot,
      ":" => Self::Colon,
      "-" => Self::FigureDash,
      "—" => Self::EnDash,
      "'" => Self::Apostrophe,
      "/" => Self::Slash,
      _ => unreachable!(),
    }
  }
}
