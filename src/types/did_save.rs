use serde_json;
use serde::{Serialize, Deserialize};

use super::did_change::VersionedTextDocumentIdentifier;

#[derive(Deserialize, Serialize)]
pub struct DidSaveNotification {
    pub jsonrpc: String,
    pub method: String,
    pub params: SaveParams
}


#[derive(Deserialize, Serialize)]
pub struct SaveParams {
    pub textDocument: VersionedTextDocumentIdentifier,
    pub text: String
}


#[derive(Deserialize, Serialize)]
pub struct ContentSaveEvent {
   pub text: String 
}



pub fn parse_save_notification(msg: &str) -> DidSaveNotification {
    let notification = serde_json::from_str(&msg);
    match notification {
        Ok(notification) => { return notification },
        Err(_) => {
            return DidSaveNotification {
                jsonrpc: String::from(msg),
                method: String::new(),
                params: SaveParams {
                    textDocument: VersionedTextDocumentIdentifier {
                        uri: String::new(),
                        version: 1,
                    },
                    text: String::new()
                }
            }
        }
    }
}




