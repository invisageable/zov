use zov_tokens::token;
use zov_tokens::token::punctuation;
use zov_tokens::token::word::Syllable;

#[derive(Clone, Debug)]
pub struct Ast {
  /// A collection of nodes.
  pub nodes: Vec<Node>,
}

impl Ast {
  pub fn new(nodes: Vec<Node>) -> Self {
    Self { nodes }
  }

  pub fn add_node(&mut self, node: Node) {
    self.nodes.push(node);
  }
}

#[derive(Clone, Debug)]
pub struct Node {
  /// A node kind.
  pub kind: NodeKind,
}

#[derive(Clone, Debug)]
pub enum NodeKind {
  /// A whitespace.
  Whitespace,
  /// A newline.
  LineBreak,
  /// A lines of multiple nodes.
  Lines(Box<Vec<Node>>),
  /// A punctuation mark.
  Punctuation(Punctuation),
  /// An apostrophe.
  Apostrophe,
  /// A number.
  Number(String),
  /// A word as an atomic unit.
  Word(Word),
}

#[derive(Clone, Debug)]
pub struct Word {
  /// A text format.
  pub text: String,
  /// A collection of syllables.
  pub syllables: Option<Vec<Syllable>>,
}

impl From<&token::word::Word> for Word {
  fn from(word: &token::word::Word) -> Self {
    Self {
      text: word.text.clone(),
      syllables: word.syllables.clone(),
    }
  }
}

#[derive(Clone, Debug)]
pub enum Punctuation {
  Dot,
}

impl From<&punctuation::Punctuation> for Punctuation {
  fn from(punctuation: &punctuation::Punctuation) -> Self {
    match punctuation {
      punctuation::Punctuation::Dot => Self::Dot,
      _ => unreachable!(),
    }
  }
}
