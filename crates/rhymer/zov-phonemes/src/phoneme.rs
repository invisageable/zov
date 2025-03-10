pub mod consonant;
pub mod vowel;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Phoneme {
  kind: PhonemeKind,
}

impl std::fmt::Display for Phoneme {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.kind)
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhonemeKind {
  Consonant(consonant::Consonant),
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
