#[derive(Debug)]
pub enum Error {
  /// An error used by the tokenizer during the lexical analysis.
  Lexical,
  /// An error used by the parser during the syntax analysis.
  Syntax,
}
