use std::path::PathBuf;

use lspower::jsonrpc::Result;
use lspower::lsp::*;
use lspower::{Client, LanguageServer, LspService, Server};

use spwn::parse_spwn;

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
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec!["."].iter().map(|v| v.to_string()).collect()),
                    ..Default::default()
                }),
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        save: Some(TextDocumentSyncSaveOptions::SaveOptions(SaveOptions {
                            include_text: Some(true),
                        })),
                        change: Some(TextDocumentSyncKind::Full),
                        ..Default::default()
                    },
                )),

                ..Default::default()
            },
        })
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

fn compute_range(text: String, (start, end): (usize, usize)) -> Range {
    let start_line_number = text.chars().take(start).collect::<String>().lines().count() - 1;
    let end_line_number = text.chars().take(end).collect::<String>().lines().count() - 1;
    let total_lines = end_line_number - start_line_number;
    let start_char = start - total_lines + 1;
    let end_char = end - total_lines + 1;

    Range {
        start: Position {
            line: start_line_number as u32,
            character: start_char as u32,
        },
        end: Position {
            line: end_line_number as u32,
            character: end_char as u32,
        },
    }
}

async fn set_syntax_errors(text: String, text_location: Url, client: Client) {
    let parsed = parse_spwn(text.clone(), PathBuf::from(text_location.path()));

    match parsed {
        Ok(_) => {
            client
                .publish_diagnostics(text_location, vec![], None)
                .await
        }

        Err(error) => match error {
            spwn::parser::SyntaxError::ExpectedErr {
                expected,
                found,
                pos,
                file: _,
            } => {
                client
                    .publish_diagnostics(
                        text_location,
                        vec![Diagnostic {
                            code: None,
                            code_description: None,
                            data: None,
                            message: format!("ERROR: expected {},\nFOUND: {}", expected, found),
                            range: compute_range(text.replace("\r\n", "\n"), pos),
                            severity: Some(DiagnosticSeverity::Error),
                            related_information: None,
                            source: Some("SPWN Syntax Error (Expected)".to_string()),
                            tags: None,
                        }],
                        None,
                    )
                    .await
            }

            spwn::parser::SyntaxError::UnexpectedErr { found, pos, .. } => {
                client
                    .publish_diagnostics(
                        text_location,
                        vec![Diagnostic {
                            code: None,
                            code_description: None,
                            data: None,
                            message: format!("ERROR: unexpected {}", found),
                            range: compute_range(text.replace("\r\n", "\n"), pos),
                            severity: Some(DiagnosticSeverity::Error),
                            related_information: None,
                            source: Some("SPWN Syntax Error (Unexpected)".to_string()),
                            tags: None,
                        }],
                        None,
                    )
                    .await
            }
            spwn::parser::SyntaxError::SyntaxError { message, pos, .. } => {
                client
                    .publish_diagnostics(
                        text_location,
                        vec![Diagnostic {
                            code: None,
                            code_description: None,
                            data: None,
                            message: format!("SYNTAX ERROR: {}", message),
                            range: compute_range(text.replace("\r\n", "\n"), pos),
                            severity: Some(DiagnosticSeverity::Error),
                            related_information: None,
                            source: Some("SPWN Syntax Error".to_string()),
                            tags: None,
                        }],
                        None,
                    )
                    .await
            }
        },
    }
}
