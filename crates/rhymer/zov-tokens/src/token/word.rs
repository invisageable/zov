//! The french word.
//!
//! @see — https://fr.wikipedia.org/wiki/Ponctuation.

use zov_interner::interner::symbol::Symbol;
use zov_phonemes::phoneme::Phoneme;

/// The representation of a word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Word {
  /// A text format.
  text: Symbol,
  /// An array of syllables.
  syllables: &'static [Syllable],
}

impl std::fmt::Display for Word {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.text)
  }
}

/// The representation of a syllable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Syllable {
  /// A text format.
  text: Symbol,
  /// An array of phonemes.
  phonemes: &'static [Phoneme],
}
