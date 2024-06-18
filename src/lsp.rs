use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Deserialize)]
pub struct Request {
    pub jsonrpc: String,
    pub method: String,
    pub id: Option<u32>
}

#[derive(Deserialize, Serialize)]
pub struct Response{
    pub jsonrpc: String,
    pub id: u32,
    pub result: InitializeResult
}


#[derive(Deserialize, Serialize)]
pub struct InitializeResult {
    pub capabilities: Capabilities,
    pub serverInfo: ServerInfo
}

#[derive(Deserialize, Serialize)]
pub struct ServerInfo {
    pub name: String
}

#[derive(Deserialize, Serialize)]
pub struct Capabilities {
    pub textDocumentSync: u8
}


#[derive(Deserialize, Serialize)]
pub struct DidOpenNotification {
    pub jsonrpc: String,
    pub method: String,
    pub params: OpenParams
}

#[derive(Deserialize, Serialize)]
pub struct OpenParams {
    pub textDocument: TextDocumentItem
}

#[derive(Deserialize, Serialize)]
pub struct TextDocumentItem {
    pub uri: String,
    pub languageId: String,
    pub version: u32,
    pub text: String
}


pub fn serialize_response(msg: impl Serialize) -> String {
    let json = serde_json::to_string(&msg).expect("failed to serialize struct");
    return format!("Content-Length: {}\r\n\r\n{}", json.len(), json);
}

pub fn parse_notification(msg: &str) -> DidOpenNotification {
    let notification = serde_json::from_str(&msg);
    match notification {
        Ok(notification) => { return notification },
        Err(_) => {
            return DidOpenNotification {
                jsonrpc: String::from(msg),
                method: String::new(),
                params: OpenParams {
                    textDocument: TextDocumentItem {
                        uri: String::new(),
                        languageId: String::from("1"),
                        version: 1,
                        text: String::new()
                    }
                }
            }
        }
    }
}

pub fn parse_request(msg: String) -> Request {
    let request = serde_json::from_str(&msg);
    match request {
        Ok(request) => { return request },
        Err(_) => { return Request {jsonrpc: msg, method: String::new(), id: Some(1)} }
    }
}
