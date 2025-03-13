use super::ast::{Node, NodeKind};

impl std::fmt::Display for Node {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.kind)
  }
}

impl std::fmt::Display for NodeKind {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Whitespace => write!(f, " "),
      Self::Newline => write!(f, "\n"),
      Self::Line(_) => write!(f, ""),
      Self::Punctuation(_) => write!(f, ""),
      Self::Apostrophe => write!(f, ""),
      Self::Number(_) => write!(f, ""),
      Self::Word(_) => write!(f, ""),
    }
  }
}
