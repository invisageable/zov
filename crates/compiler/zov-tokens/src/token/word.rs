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
  pub syllables: Option<Vec<Syllable>>, // ["bon", "jour"]
  /// A phoneme.
  pub phoneme: Option<Phoneme>,
}

impl Word {
  /// Creates a new [`Word`] instance.
  pub fn new(
    text: String,
    // syllables: Option<&'static [Syllable]>
  ) -> Self {
    Self {
      text,
      syllables: None,
      phoneme: None,
    }
  }

  pub fn add_phoneme(&mut self, phoneme: Phoneme) {
    self.phoneme = Some(phoneme);
  }

  pub fn add_syllables(&mut self, syllables: Vec<Syllable>) {
    self.syllables = Some(syllables);
  }
}

impl std::fmt::Display for Word {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.text)
  }
}

// #[derive(Debug, Clone)]
// pub struct Word {
//   pub text: String,
//   pub syllables: Vec<Syllabe>,
// }

/// The representation of a syllable.
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub struct Syllable {
//   /// A text format.
//   text: Symbol,
//   /// An array of phonemes.
//   phonemes: &'static [Phoneme],
// }

/// The representation of a syllable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Syllable {
  /// A text format.
  pub text: String, // bon.
  /// An array of phonemes.
  pub phoneme: String, // "bɔ̃"
}

impl Syllable {
  /// ...
  pub fn new(text: String, phoneme: String) -> Self {
    Self { text, phoneme }
  }
}
