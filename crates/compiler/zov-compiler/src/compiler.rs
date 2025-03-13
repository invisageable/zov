use zov_reporter::Result;

#[derive(Debug)]
pub enum Phase {
  Tokenizing,
  Phonemizing,
  Parsing,
}

/// The representation of a compiler.
#[derive(Debug)]
pub struct Compiler<const L: usize> {
  /// The compiler's phases.
  phases: [Phase; L],
}

impl<const L: usize> Compiler<L> {
  /// Creates a new [`Compiler`] instance.
  pub fn new(phases: [Phase; L]) -> Self {
    Self { phases }
  }

  /// Executes the compiler phases.
  pub fn compile(&mut self) -> Result<()> {
    Ok(())
  }
}

impl<const L: usize> From<[Phase; L]> for Compiler<L> {
  #[inline]
  fn from(phases: [Phase; L]) -> Self {
    Self::new(phases)
  }
}
