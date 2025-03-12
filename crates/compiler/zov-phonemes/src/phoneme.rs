//! The french phonemes.
//!
//! @see — https://mortain.circonscription.ac-normandie.fr/IMG/pdf/3_phonemes_francais.pdf.
//! @see — https://mortain.circonscription.ac-normandie.fr/IMG/pdf/2._systeme_francais.pdf.

pub mod consonant;
pub mod vowel;

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub struct Phoneme {
//   /// A phoneme kind.
//   kind: PhonemeKind,
// }

/// The representation of a phoneme.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Phoneme {
  // pub kind: PhonemeKind,
  /// A text format.
  pub text: String, // bɔ̃ʒuʁ.
}

impl Phoneme {
  pub fn new(text: String) -> Self {
    Self { text }
  }
}

impl std::fmt::Display for Phoneme {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.text)
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhonemeKind {
  /// A consonant phoneme kind.
  Consonant(consonant::Consonant),
  /// A vowel phoneme kind.
  Vowel(vowel::Vowel),
}

impl std::fmt::Display for PhonemeKind {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Consonant(consonant) => write!(f, "{consonant}"),
      Self::Vowel(vowel) => write!(f, "{vowel}"),
    }
  }
}
