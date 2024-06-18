
use serde::{Serialize, Deserialize};
use serde_json;
pub mod initialization;
pub mod did_open;
pub mod diagnostics;
pub mod did_change;
pub mod did_save;

#[derive(Deserialize)]
pub struct Request {
    pub jsonrpc: String,
    pub method: String,
    pub id: Option<u32>
}


#[derive(Serialize, Debug)]
pub struct Range {
    pub start: Position,
    pub end: Position
}


#[derive(Serialize, Debug)]
pub struct Position{
    pub line: u32,
    pub character: u32
}


pub fn parse_request(msg: String) -> Request {
    let request = serde_json::from_str(&msg);
    match request {
        Ok(request) => { return request },
        Err(_) => { return Request {jsonrpc: msg, method: String::new(), id: Some(1)} }
    }
}


pub fn serialize_response(msg: impl Serialize) -> String {
    let json = serde_json::to_string(&msg).expect("failed to serialize struct");
    return format!("Content-Length: {}\r\n\r\n{}", json.len(), json);
}


pub fn parse_any<'a, T: Deserialize<'a>>(msg: &'a str) -> Result<T, String> {
    let result = serde_json::from_str(&msg);
    match result {
        Ok(result) => { return Ok(result); },
        Err(_) => { return Err(format_args!("failed to parse: {}\n\n", msg).to_string()) }
    };
}


