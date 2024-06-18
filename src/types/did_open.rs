use serde::{Serialize, Deserialize};
use serde_json;


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

pub fn parse_open_notification(msg: &str) -> DidOpenNotification {
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
