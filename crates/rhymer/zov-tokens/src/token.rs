//! The token.

pub mod group;
pub mod punctuation;
pub mod quote;
pub mod word;

/// The representation of a token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
  /// A token kind.
  kind: TokenKind,
}

impl std::fmt::Display for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.kind)
  }
}

/// The representation of a token kind.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
  /// An end of file token kind.
  #[default]
  Eof,
  /// A group token kind.
  Group(group::Group),
  /// A newline token kind.
  Newline,
  /// A punctuation token kind.
  Punctuation(punctuation::Punctuation),
  /// A quote token kind.
  Quote(quote::Quote),
  /// A space token kind.
  Space,
  /// A word token kind.
  Word(word::Word),
}

impl std::fmt::Display for TokenKind {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Eof => write!(f, "eof"),
      Self::Group(group) => write!(f, "{group}"),
      Self::Newline => write!(f, "newline"),
      Self::Punctuation(punctuation) => write!(f, "{punctuation}"),
      Self::Quote(quote) => write!(f, "{quote}"),
      Self::Space => write!(f, "space"),
      Self::Word(word) => write!(f, "{word}"),
    }
  }
}
