mod initialize;
mod syntax_errors;
mod utils;

use initialize::create_init;
use lspower::jsonrpc::Result;
use lspower::lsp::*;
use lspower::{Client, LanguageServer, LspService, Server};
use syntax_errors::set_syntax_errors;

#[derive(Debug)]
struct Backend {
    client: Client,
}

#[lspower::async_trait]
impl LanguageServer for Backend {
    async fn completion(
        &self,
        _params: lsp_types::CompletionParams,
    ) -> lspower::jsonrpc::Result<Option<lsp_types::CompletionResponse>> {
        Ok(Some(CompletionResponse::Array(vec![
            CompletionItem::new_simple("to-be-added".to_string(), "haha yes".to_string()),
        ])))
    }

    async fn did_change(&self, params: lsp_types::DidChangeTextDocumentParams) {
        set_syntax_errors(
            params.content_changes.first().unwrap().text.clone(),
            params.text_document.uri,
            self.client.clone(),
        )
        .await
    }

    async fn did_save(&self, params: lsp_types::DidSaveTextDocumentParams) {
        set_syntax_errors(
            params.text.unwrap(),
            params.text_document.uri,
            self.client.clone(),
        )
        .await
    }

    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(create_init())
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::Info, "SPWN-LSP initialized!")
            .await;
    }

    async fn shutdown(&self) -> lspower::jsonrpc::Result<()> {
        Ok(())
    }
}
#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, messages) = LspService::new(|client| Backend { client });
    Server::new(stdin, stdout)
        .interleave(messages)
        .serve(service)
        .await;
}

