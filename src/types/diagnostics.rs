use super::Range;
use serde::Serialize;

#[derive(Serialize)]
pub struct PublishDiagnosticsParams {
    pub uri: String,
    pub version: u32,
    pub diagnostics: Vec<Diagnostic>
}

#[derive(Serialize, Debug)]
pub struct Diagnostic {
    pub range: Range,
    pub message: String,
    pub severity: u32
}


#[derive(Serialize)]
pub struct PublishDiagnosticsNotification{
    pub jsonrpc: String,
    pub method: String,
    pub params: PublishDiagnosticsParams
}

