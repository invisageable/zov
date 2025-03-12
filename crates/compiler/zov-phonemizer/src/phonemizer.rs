use zov_phonemes::phoneme::Phoneme;
use zov_reporter::Result;
use zov_tokens::token::word::Syllable;
use zov_tokens::token::{Token, TokenKind};

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

#[derive(Debug)]
pub struct Phonemizer {}

impl Phonemizer {
  /// ...
  pub fn new() -> Self {
    Self {}
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

        // println!("{:?}", words);

        dict.set_item("lines", words).unwrap();

        // let phonemes: Vec<String> =
        //   phonemize_french.call1((dict,)).unwrap().extract().unwrap();

        let phonemes: Vec<(String, String)> =
          phonemize_french.call1((dict,)).unwrap().extract().unwrap();

        println!("{:?}", phonemes);

        todo!()
        // phonemes.into()
      });

    for token in tokens.iter_mut() {
      if let TokenKind::Word(word) = &mut token.kind
        && let Some(phoneme) = phonemes.pop_front()
      {
        let syllables = self.syllabilize(word.to_string(), phoneme.clone())?;

        word.add_syllables(syllables);
        word.add_phoneme(Phoneme::new(phoneme));
      }
    }

    Ok(tokens)
  }

  /// ...
  fn syllabilize(
    &mut self,
    word: String,
    phoneme: String,
  ) -> Result<Vec<Syllable>> {
    let syllables = Vec::with_capacity(0usize);

    let _ = self.syllabilize_word(&word)?;
    let phonemes = self.syllabilize_phoneme(&phoneme)?;

    println!("{} | {}", word, phoneme);
    // println!("SYLLABLES.PHONEMES({:?})", phonemes);

    Ok(syllables)
  }

  /// ...
  fn syllabilize_word(&mut self, word: &str) -> Result<Vec<String>> {
    let mut syllables: Vec<String> = Vec::with_capacity(0usize);
    let mut current_syllables = Vec::with_capacity(0usize);

    for ch in word.split("") {
      current_syllables.push(ch);
    }

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
          syllables.push(current_syllable.join(" "));
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
      syllables.push(current_syllable.join(" "));
    }

    Ok(syllables)
  }
}

/// ...
pub fn phonemize(tokens: Vec<Token>) -> Result<Vec<Token>> {
  Phonemizer::new().phonemize(tokens)
}
