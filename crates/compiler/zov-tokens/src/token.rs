//! The token.

pub mod group;
pub mod punctuation;
pub mod quote;
pub mod word;

/// The representation of a token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
  /// A token kind.
  pub kind: TokenKind,
}

impl Token {
  pub const EOF: Self = Self::new(TokenKind::Eof);

  pub const fn new(kind: TokenKind) -> Self {
    Self { kind }
  }

  /// Checks if the kind of a token matched from a other token kind.
  #[inline]
  pub fn is(&self, kind: TokenKind) -> bool {
    self.kind.is(kind)
  }
}

impl std::fmt::Display for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.kind)
  }
}

/// The representation of a token kind.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum TokenKind {
  /// An end of file token kind.
  #[default]
  Eof,
  /// A space token kind.
  Space,
  /// A newline token kind.
  Newline,
  /// A group token kind.
  Group(group::Group),
  /// A punctuation token kind.
  Punctuation(punctuation::Punctuation),
  /// A quote token kind.
  Quote(quote::Quote),
  /// A word token kind.
  Word(word::Word),
  // Ident(String),
}

impl TokenKind {
  pub fn is(&self, kind: Self) -> bool {
    *self == kind
  }
}

impl std::fmt::Display for TokenKind {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Eof => write!(f, "eof"),
      Self::Space => write!(f, "space"),
      Self::Newline => write!(f, "newline"),
      Self::Group(group) => write!(f, "{group}"),
      Self::Punctuation(punctuation) => write!(f, "{punctuation}"),
      Self::Quote(quote) => write!(f, "{quote}"),
      Self::Word(word) => write!(f, "{word}"),
      // Self::Ident(word) => write!(f, "{word}"),
    }
  }
}
