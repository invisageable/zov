pub mod build;

use clap::command;
use clap_derive::{Parser, Subcommand};

/// A behavior to execute a command.
pub(crate) trait Execute {
  /// Executes the command.
  fn exec(&self);
}

/// The representation of a command.
#[derive(Parser)]
#[clap(about, author, name = "\nzo")]
pub(crate) struct Cmd {
  /// A comand.
  #[command(subcommand)]
  command: Command,
}

impl Cmd {
  /// Creates a new cmd.
  #[inline]
  pub fn run(&mut self) {
    self.cmd()
  }

  /// Executes a command.
  #[inline]
  fn cmd(&mut self) {
    match self.command {
      Command::Build(ref command) => command.exec(),
    }
  }
}

/// The representation of a command kind.
#[derive(Subcommand)]
pub(crate) enum Command {
  /// Builds a program.
  Build(build::Build),
}
