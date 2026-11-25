//! The Model Context Protocol, as far as tools need it.

use serde_json::{Value, json};

use super::tools;

const VERSIONS: [&str; 3] = ["2025-06-18", "2025-03-26", "2024-11-05"];
const PARSE_ERROR: i32 = -32700;
const INVALID_REQUEST: i32 = -32600;
const UNKNOWN_METHOD: i32 = -32601;
const INVALID_PARAMS: i32 = -32602;

#[derive(Debug, PartialEq)]
pub enum Incoming {
  Nothing,
  Reply(Value),
  Call { id: Value, name: String, arguments: Value },
  Cancel(Value),
}

fn result(id: Value, result: Value) -> Value {
  json!({ "jsonrpc": "2.0", "id": id, "result": result })
}

fn error(id: Value, code: i32, message: &str) -> Incoming {
  Incoming::Reply(json!({ "jsonrpc": "2.0", "id": id, "error": { "code": code, "message": message } }))
}

/// Failures of a tool are results, so the assistant can read and fix them.
pub fn tool_result(id: Value, outcome: Result<Value, String>) -> Value {
  let is_error = outcome.is_err();
  let text = outcome.map_or_else(|message| message, |value| value.to_string());
  result(id, json!({ "content": [{ "type": "text", "text": text }], "isError": is_error }))
}

fn initialize(id: Value, params: &Value) -> Incoming {
  let asked = params["protocolVersion"].as_str();
  let version = VERSIONS.into_iter().find(|version| asked == Some(version)).unwrap_or(VERSIONS[0]);
  Incoming::Reply(result(
    id,
    json!({
      "protocolVersion": version,
      "capabilities": { "tools": {} },
      "serverInfo": { "name": "voxmelody", "title": "VoxMelody", "version": env!("CARGO_PKG_VERSION") },
      "instructions": "Generates speech, instrumental music, songs and mixes on this computer. Results are WAV files saved to the VoxMelody library."
    }),
  ))
}

fn call(id: Value, params: &mut Value) -> Incoming {
  match params["name"].as_str().map(String::from) {
    Some(name) if tools::exists(&name) => {
      let arguments = if params["arguments"].is_object() { params["arguments"].take() } else { json!({}) };
      Incoming::Call { id, name, arguments }
    }
    _ => error(id, INVALID_PARAMS, "Unknown tool"),
  }
}

pub fn read(line: &str) -> Incoming {
  if line.trim().is_empty() {
    return Incoming::Nothing;
  }
  let Ok(mut message) = serde_json::from_str::<Value>(line) else {
    return error(Value::Null, PARSE_ERROR, "Invalid JSON");
  };
  if !message.is_object() {
    return error(Value::Null, INVALID_REQUEST, "Invalid request");
  }
  let id = message["id"].take();
  let mut params = message["params"].take();
  let Some(method) = message["method"].as_str() else {
    // Replies are not expected, because the server asks nothing.
    return Incoming::Nothing;
  };
  match method {
    "notifications/cancelled" if params.is_object() => Incoming::Cancel(params["requestId"].take()),
    _ if id.is_null() => Incoming::Nothing,
    "initialize" => initialize(id, &params),
    "ping" => Incoming::Reply(result(id, json!({}))),
    "tools/list" => Incoming::Reply(result(id, json!({ "tools": tools::definitions() }))),
    "tools/call" if params.is_object() => call(id, &mut params),
    "tools/call" => error(id, INVALID_PARAMS, "Unknown tool"),
    _ => error(id, UNKNOWN_METHOD, "Unknown method"),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn reply(line: &str) -> Value {
    match read(line) {
      Incoming::Reply(value) => value,
      other => panic!("{other:?}"),
    }
  }

  #[test]
  fn agrees_on_a_version() {
    let ask = |version: &str| {
      let line = json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":version}});
      reply(&line.to_string())["result"]["protocolVersion"].clone()
    };
    assert_eq!(ask("2024-11-05"), "2024-11-05");
    assert_eq!(ask("1999-01-01"), VERSIONS[0]);
    assert_eq!(reply(r#"{"id":1,"method":"initialize"}"#)["result"]["serverInfo"]["name"], "voxmelody");
  }

  #[test]
  fn lists_and_calls_tools() {
    let tools = reply(r#"{"jsonrpc":"2.0","id":"a","method":"tools/list"}"#);
    assert_eq!(tools["id"], "a");
    for tool in tools["result"]["tools"].as_array().unwrap() {
      assert!(tools::exists(tool["name"].as_str().unwrap()));
      assert_eq!(tool["inputSchema"]["type"], "object");
    }
    let line = r#"{"id":2,"method":"tools/call","params":{"name":"generate_speech","arguments":{"text":"Hi"}}}"#;
    assert_eq!(
      read(line),
      Incoming::Call { id: json!(2), name: "generate_speech".into(), arguments: json!({ "text": "Hi" }) }
    );
    assert_eq!(reply(r#"{"id":2,"method":"tools/call","params":{"name":"nope"}}"#)["error"]["code"], INVALID_PARAMS);
    assert_eq!(reply(r#"{"id":2,"method":"tools/call"}"#)["error"]["code"], INVALID_PARAMS);
  }

  #[test]
  fn handles_notifications_and_bad_input() {
    assert_eq!(read(""), Incoming::Nothing);
    assert_eq!(read(r#"{"method":"notifications/initialized"}"#), Incoming::Nothing);
    assert_eq!(read(r#"{"id":1,"result":{}}"#), Incoming::Nothing);
    assert_eq!(
      read(r#"{"method":"notifications/cancelled","params":{"requestId":7}}"#),
      Incoming::Cancel(json!(7))
    );
    assert_eq!(reply("{broken")["error"]["code"], PARSE_ERROR);
    assert_eq!(reply("[]")["error"]["code"], INVALID_REQUEST);
    assert_eq!(reply(r#"{"id":1,"method":"resources/list"}"#)["error"]["code"], UNKNOWN_METHOD);
  }

  #[test]
  fn tool_failures_are_readable() {
    let failed = tool_result(json!(1), Err("No".into()));
    assert_eq!(failed["result"]["isError"], true);
    assert_eq!(failed["result"]["content"][0]["text"], "No");
  }
}
