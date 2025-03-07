use lsp_types::{
    CompletionOptions, InitializeResult, SaveOptions, ServerCapabilities, ServerInfo,
    TextDocumentSyncCapability, TextDocumentSyncKind, TextDocumentSyncOptions,
    TextDocumentSyncSaveOptions,
};

pub fn create_init() -> InitializeResult {
    let server_info = ServerInfo {
        name: "SPWN-Srvr".to_string(),
        version: Some("0.1.0".to_string()),
    };

    let completion_provider = CompletionOptions {
        resolve_provider: Some(false),
        trigger_characters: Some(vec!["."].iter().map(|v| v.to_string()).collect()),
        ..Default::default()
    };

    let text_document_sync = TextDocumentSyncCapability::Options(TextDocumentSyncOptions {
        change: Some(TextDocumentSyncKind::Full),
        open_close: Some(true),
        save: Some(TextDocumentSyncSaveOptions::SaveOptions(SaveOptions {
            include_text: Some(true),
        })),
        will_save: Some(false),
        will_save_wait_until: Some(false),
    });

    let capabilities = ServerCapabilities {
        completion_provider: Some(completion_provider),
        text_document_sync: Some(text_document_sync),
        ..Default::default()
    };

    InitializeResult {
        capabilities,
        server_info: Some(server_info),
    }
}

mod tests {
    #[test]
    fn is_correct_version() {
        use super::create_init;

        let output = create_init();
        let current_version = env!("CARGO_PKG_VERSION");

        assert_eq!(
            output.server_info.unwrap().version.unwrap(),
            current_version
        )
    }
}
