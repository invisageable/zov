use zov_phonemes::phoneme::Phoneme;
use zov_reporter::Result;
use zov_tokens::token::word::Syllable;
use zov_tokens::token::{Token, TokenKind};

use hyphenation::{Hyphenator, Iter, Language, Load, Standard};
use pyo3::ffi::c_str;
use pyo3::types::{PyAnyMethods, PyDict, PyModule};
use pyo3::Python;
use unicode_normalization::UnicodeNormalization;

const PHONEME_VOWELS: &[&'static str] = &[
  "a", "ɑ", "ã", "ə", "e", "ɛ", "ɛ̃", "œ", "œ̃", "ø", "o", "ɔ", "ɔ̃", "i", "y",
  "u",
];

const PHONEME_CONSONANTS: &[&'static str] = &[
  "p", "f", "t", "s", "ʃ", "k", "b", "v", "d", "z", "ʒ", "g", "m", "n", "l",
  "r", "ɲ", "j", "w", "ɥ",
];

/// The representation of a Phonemizer.
#[derive(Debug)]
pub struct Phonemizer {
  /// A hyphenator.
  hyphenator: Standard,
}

impl Phonemizer {
  /// Creates a new [`Phonemizer`] instance.
  pub fn new() -> Self {
    Self {
      hyphenator: Standard::from_embedded(Language::French).unwrap(),
    }
  }

  fn is_phoneme_vowel(&self, phoneme: &str) -> bool {
    PHONEME_VOWELS.contains(&phoneme)
  }

  fn is_phoneme_consonant(&self, phoneme: &str) -> bool {
    PHONEME_CONSONANTS.contains(&phoneme)
  }

  /// ...
  pub fn phonemize(&mut self, mut tokens: Vec<Token>) -> Result<Vec<Token>> {
    let mut phonemes: std::collections::VecDeque<String> =
      Python::with_gil(|py| {
        let code = c_str!(include_str!("../main.py"));
        let file_name = c_str!("phonemizer_wrapper.py");
        let module_name = c_str!("phonemizer_wrapper");

        let phonemizer_module =
          PyModule::from_code(py, code, file_name, module_name).unwrap();

        let phonemize_french =
          phonemizer_module.getattr("phonemize_french").unwrap();

        let dict = PyDict::new(py);

        let words = tokens
          .clone()
          .into_iter()
          .filter_map(|token| match token.kind {
            TokenKind::Word(word) => Some(word.to_string()),
            _ => None,
          })
          .collect::<Vec<_>>();

        dict.set_item("lines", words).unwrap();

        let phonemes: Vec<String> =
          phonemize_french.call1((dict,)).unwrap().extract().unwrap();

        phonemes.into()
      });

    for token in tokens.iter_mut() {
      if let TokenKind::Word(word) = &mut token.kind
        && let Some(phoneme) = phonemes.pop_front()
      {
        let syllables = self.syllabilize(word.to_string(), &phoneme)?;

        word.add_syllables(Some(syllables));
        word.add_phoneme(Some(Phoneme::new(phoneme)));
      }
    }

    Ok(tokens)
  }

  /// ...
  fn syllabilize(
    &mut self,
    word: String,
    phoneme: &str,
  ) -> Result<Vec<Syllable>> {
    let mut syllables = Vec::with_capacity(0usize);
    let syllables_words = self.syllabilize_word(&word)?;
    let _syllables_phonemes = self.syllabilize_phoneme(&phoneme)?;

    // note(ivs) — ensure that `syllables_words` and `syllables_phonemes` have
    // the same length.
    for (_idx, syllable) in syllables_words.into_iter().enumerate() {
      syllables.push(Syllable {
        text: syllable,
        phoneme: String::with_capacity(0usize), // syllables_phonemes[idx]
      });
    }

    println!("{} | {}", word, phoneme);

    Ok(syllables)
  }

  /// ...
  fn syllabilize_word(&mut self, word: &str) -> Result<Vec<String>> {
    let hyphenated = self.hyphenator.hyphenate(word);
    let segments = hyphenated.iter().segments();
    let syllables: Vec<String> = segments.map(|s| s.to_string()).collect();

    Ok(syllables)
  }

  /// ...
  fn syllabilize_phoneme(&mut self, phoneme: &str) -> Result<Vec<String>> {
    let mut syllables: Vec<String> = Vec::with_capacity(0usize);
    let mut current_syllable: Vec<String> = Vec::with_capacity(0usize);
    let phonemes = phoneme.nfc().map(String::from).collect::<Vec<_>>();

    for (idx, phoneme) in phonemes.iter().enumerate() {
      current_syllable.push(phoneme.into());

      let next_syllable = phonemes.get(idx + 1);

      if let Some(next_phoneme) = next_syllable {
        let is_vowel = self.is_phoneme_vowel(&next_phoneme.as_str());
        let is_consonant = !is_vowel;

        if is_vowel {
          syllables.push(current_syllable.join(""));
          current_syllable.clear();
        } else if is_consonant {
          if let Some(after_next_syllable) = phonemes.get(idx + 2) {
            if self.is_phoneme_vowel(after_next_syllable) {
              syllables.push(current_syllable.join(""));
              current_syllable.clear();
            }
          }
        }
      }
    }

    if !current_syllable.is_empty() {
      syllables.push(current_syllable.join(""));
    }

    Ok(syllables)
  }
}

/// ...
pub fn phonemize(tokens: Vec<Token>) -> Result<Vec<Token>> {
  Phonemizer::new().phonemize(tokens)
}
