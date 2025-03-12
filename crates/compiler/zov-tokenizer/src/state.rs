#[derive(Debug)]
pub enum State {
  /// A data state.
  Data,
  /// A space state.
  Space,
  /// A newline state.
  Newline,
  /// A number state.
  Num,
  /// A group state.
  Group,
  /// A punctuation state.
  Punctuation,
  /// A quote state.
  Quote,
  /// An word state.
  Word,
  /// An unknown state.
  Unknown,
}
