pub mod compiler;

use zov_parser::parser;
use zov_phonemizer::phonemizer;
use zov_reporter::Result;
use zov_tokenizer::tokenizer;

#[derive(Debug)]
pub struct Token {
  text: String,
  phoneme: String,
}

pub fn compile(source: &str) -> Result<()> {
  // --- TOKENiZiNG.

  let tokens = tokenizer::tokenize(source)?;

  println!("\n{:?}\n", tokens);

  // --- PHONEMiZiNG.

  let tokens = phonemizer::phonemize(tokens)?;

  println!("\n{:?}\n", tokens);

  // --- PARSiNG.

  let ast = parser::parse(tokens)?;

  println!("\n{:?}\n", ast);

  Ok(())
}
