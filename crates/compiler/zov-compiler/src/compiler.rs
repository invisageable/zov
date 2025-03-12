use zov_reporter::Result;

#[derive(Debug)]
pub enum Phase {
  Tokenizing,
  Phonemizing,
  Parsing,
}

#[derive(Debug)]
pub struct Compiler<const L: usize> {
  /// The compiler's phases.
  phases: [Phase; L],
}

impl<const L: usize> Compiler<L> {
  pub fn new(phases: [Phase; L]) -> Self {
    Self { phases }
  }

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
