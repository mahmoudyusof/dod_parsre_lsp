use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct InitializationRequest {
    pub jsonrpc: String,
    pub id: Option<u32>,
    pub method: String,
}


#[derive(Deserialize, Serialize)]
pub struct InitializationResponse {
    pub jsonrpc: String,
    pub id: u32,
    pub result: InitializeResult,
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
    pub textDocumentSync: u8,
    pub documentHighlightProvider: bool,
    pub colorProvider: bool,
}


#[derive(Deserialize, Serialize)]
pub struct DiagnosticOptions {
    pub identifier: String,
    pub interFileDependencies: bool,
    pub workspaceDiagnostics: bool

}




