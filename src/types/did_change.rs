use serde::{Serialize, Deserialize};
use serde_json;


#[derive(Deserialize, Serialize)]
pub struct DidChangeNotification {
    pub jsonrpc: String,
    pub method: String,
    pub params: ChangeParams
}


#[derive(Deserialize, Serialize)]
pub struct ChangeParams {
    pub textDocument: VersionedTextDocumentIdentifier,
    pub contentChanges: Vec<ContentChangeEvent>
}


#[derive(Deserialize, Serialize)]
pub struct ContentChangeEvent {
   pub text: String 
}


#[derive(Deserialize, Serialize)]
pub struct VersionedTextDocumentIdentifier {
    pub uri: String,
    pub version: u32,
}

pub fn parse_change_notification(msg: &str) -> DidChangeNotification {
    let notification = serde_json::from_str(&msg);
    match notification {
        Ok(notification) => { return notification },
        Err(_) => {
            return DidChangeNotification {
                jsonrpc: String::from(msg),
                method: String::new(),
                params: ChangeParams {
                    textDocument: VersionedTextDocumentIdentifier {
                        uri: String::new(),
                        version: 1,
                    },
                    contentChanges: vec![]
                }
            }
        }
    }
}



