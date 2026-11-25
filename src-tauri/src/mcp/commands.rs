use super::ARGUMENT;

/// The command that adds the app to Claude Code. It needs no `--`, which PowerShell removes.
#[tauri::command]
pub fn assistant_command() -> Option<String> {
  let program = std::env::current_exe().ok()?;
  Some(format!("claude mcp add --scope user voxmelody \"{}\" {ARGUMENT}", program.display()))
}
