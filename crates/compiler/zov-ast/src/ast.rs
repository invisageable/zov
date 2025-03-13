use zov_phonemes::phoneme::Phoneme;
use zov_tokens::token;
use zov_tokens::token::punctuation;
use zov_tokens::token::word::Syllable;

/// The representation of an Abstract Syntax Tree (AST).
#[derive(Clone, Debug)]
pub struct Ast {
  /// A collection of nodes.
  pub nodes: Vec<Node>,
}

impl Ast {
  /// Creates a new [`Ast`] instance.
  pub fn new(nodes: Vec<Node>) -> Self {
    Self { nodes }
  }

  /// Adds a node to the collection.
  pub fn add_node(&mut self, node: Node) {
    self.nodes.push(node);
  }
}

/// ...
#[derive(Clone, Debug)]
pub struct Node {
  /// A node kind.
  pub kind: NodeKind,
}

/// ...
#[derive(Clone, Debug)]
pub enum NodeKind {
  /// A whitespace.
  Whitespace,
  /// A newline.
  Newline,
  /// A line of multiple nodes.
  Line(Box<Vec<Node>>),
  /// A punctuation mark.
  Punctuation(Punctuation),
  /// An apostrophe.
  Apostrophe,
  /// A number.
  Number(String),
  /// A word as an atomic unit.
  Word(Word),
}

/// ...
#[derive(Clone, Debug)]
pub struct Word {
  /// A text format.
  pub text: String,
  /// A collection of syllables.
  pub syllables: Option<Vec<Syllable>>,
  /// A collection of syllables.
  pub phoneme: Option<Phoneme>,
}

impl From<&token::word::Word> for Word {
  fn from(word: &token::word::Word) -> Self {
    Self {
      text: word.text.clone(),
      syllables: word.syllables.clone(),
      phoneme: word.phoneme.clone(),
    }
  }
}

/// ...
#[derive(Clone, Debug)]
pub enum Punctuation {
  Dot,
  Semi,
  Comma,
  Question,
  Exclamation,
  DotDotDot,
  Colon,
  FigureDash,
  EnDash,
  Apostrophe,
  Slash,
}

impl From<&punctuation::Punctuation> for Punctuation {
  fn from(punctuation: &punctuation::Punctuation) -> Self {
    match punctuation {
      punctuation::Punctuation::Dot => Self::Dot,
      punctuation::Punctuation::Semi => Self::Semi,
      punctuation::Punctuation::Comma => Self::Comma,
      punctuation::Punctuation::Question => Self::Question,
      punctuation::Punctuation::Exclamation => Self::Exclamation,
      punctuation::Punctuation::DotDotDot => Self::DotDotDot,
      punctuation::Punctuation::Colon => Self::Colon,
      punctuation::Punctuation::FigureDash => Self::FigureDash,
      punctuation::Punctuation::EnDash => Self::EnDash,
      punctuation::Punctuation::Apostrophe => Self::Apostrophe,
      punctuation::Punctuation::Slash => Self::Slash,
    }
  }
}
