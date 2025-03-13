//! The french word.
//!
//! @see — https://fr.wikipedia.org/wiki/Ponctuation.

// use zov_interner::interner::symbol::Symbol;
use zov_phonemes::phoneme::Phoneme;

/// The representation of a word.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Word {
  /// A text format.
  pub text: String,
  /// An array of syllables.
  pub syllables: Option<Vec<Syllable>>,
  /// A phoneme.
  pub phoneme: Option<Phoneme>,
}

impl Word {
  /// Creates a new [`Word`] instance.
  pub fn new(text: String) -> Self {
    Self {
      text,
      syllables: None,
      phoneme: None,
    }
  }

  /// ...
  pub fn add_phoneme(&mut self, phoneme: Option<Phoneme>) {
    self.phoneme = phoneme;
  }

  /// ...
  pub fn add_syllables(&mut self, syllables: Option<Vec<Syllable>>) {
    self.syllables = syllables;
  }
}

impl std::fmt::Display for Word {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.text)
  }
}

/// The representation of a syllable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Syllable {
  /// A text format.
  pub text: String,
  /// An array of phonemes.
  pub phoneme: String,
}

impl Syllable {
  /// Creates a new [`Syllable`] instance.
  pub fn new(text: String, phoneme: String) -> Self {
    Self { text, phoneme }
  }
}
