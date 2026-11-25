//! Lets an assistant such as Claude Code use the app over stdin and stdout.
//! Started with `voxmelody mcp`; opens no window and no network connection.

pub mod commands;
mod protocol;
mod tools;

use std::collections::HashMap;
use std::io::{BufRead, Write};
use std::sync::{Arc, Mutex};

use serde_json::Value;
use tauri::{AppHandle, Manager};

use protocol::Incoming;

pub const ARGUMENT: &str = "mcp";

pub fn is_requested() -> bool {
  std::env::args().nth(1).is_some_and(|argument| argument == ARGUMENT)
}

fn send(message: &Value) {
  let mut stdout = std::io::stdout().lock();
  let _ = writeln!(stdout, "{message}");
  let _ = stdout.flush();
}

/// Tools that are running, by request id.
type Running = Arc<Mutex<HashMap<String, String>>>;

fn with<T>(running: &Running, action: impl FnOnce(&mut HashMap<String, String>) -> T) -> T {
  action(&mut running.lock().unwrap_or_else(|poisoned| poisoned.into_inner()))
}

/// Answers requests until the client closes the connection, then quits.
pub fn serve(app: AppHandle) {
  std::thread::spawn(move || {
    let running = Running::default();
    for line in std::io::stdin().lock().lines().map_while(Result::ok) {
      receive(&app, &running, &line);
    }
    app.exit(0);
  });
}

fn receive(app: &AppHandle, running: &Running, line: &str) {
  match protocol::read(line) {
    Incoming::Nothing => {}
    Incoming::Reply(message) => send(&message),
    Incoming::Cancel(id) => match with(running, |running| running.remove(&id.to_string())).as_deref() {
      Some(tools::SPEECH) => app.state::<crate::voice::engine::VoiceEngine>().cancel(),
      Some(tools::MUSIC) => app.state::<crate::music::engine::MusicEngine>().cancel(),
      _ => {}
    },
    Incoming::Call { id, name, arguments } => {
      let (app, running) = (app.clone(), running.clone());
      with(&running, |running| running.insert(id.to_string(), name.clone()));
      tauri::async_runtime::spawn(async move {
        let result = tools::call(&app, &name, arguments).await;
        with(&running, |running| running.remove(&id.to_string()));
        send(&protocol::tool_result(id, result));
      });
    }
  }
}
