use super::Execute;

use zov_compiler::compiler::Compiler;
use zov_reporter::Result;

use clap_derive::Parser;

#[derive(Parser)]
pub struct Build {}

impl Build {
  /// Executes the `build` command.
  #[inline]
  fn build(&self) -> Result<()> {
    self.building()
  }

  /// Builds a program.
  fn building(&self) -> Result<()> {
    // let mut compiler = Compiler::new([]);

    let lyrics = r#"
Bonjour pour toujours mon amour.
On lance tout dans les airs.
On enterre les dieux.
"#;

    //   let lyrics = r#"
    // On lance tout dans les airs.
    // On enterre les vœux.
    // Entre la tête et la terre.
    // Ne se jouent que des jeux.

    // Regarde les armes de la garde.
    // "#;

    zov_compiler::compile(lyrics)

    // match compiler.compile()? {
    //   _ => {}
    // }
  }
}

impl Execute for Build {
  fn exec(&self) {
    match self.build() {
      Ok(_) => std::process::exit(0),
      Err(_) => std::process::exit(1),
    }
  }
}
