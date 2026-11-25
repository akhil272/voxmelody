//! Shared by every engine process.

use std::path::{Path, PathBuf};
use std::process::Stdio;

use tokio::process::Command;

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

/// Engines sit beside the app executable.
pub fn path(name: &str) -> std::io::Result<PathBuf> {
  let program = std::env::current_exe()?;
  Ok(program.with_file_name(format!("{name}{}", std::env::consts::EXE_SUFFIX)))
}

/// A hidden process that takes requests on stdin and stops with the app.
pub fn command(program: &Path) -> Command {
  let mut command = Command::new(program);
  command.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::null()).kill_on_drop(true);
  #[cfg(windows)]
  command.creation_flags(CREATE_NO_WINDOW);
  command
}
