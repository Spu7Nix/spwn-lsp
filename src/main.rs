use std::borrow::Borrow;
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

    async fn did_save(&self, params: lsp_types::DidSaveTextDocumentParams) {
        let text = params.text.unwrap();
        let parsed = parse_spwn(text.clone(), PathBuf::from(params.text_document.uri.path()));

        match parsed {
            Ok(_val) => {
                self.client
                    .log_message(MessageType::Info, "no error warnings!")
                    .await;
            }

            Err(error) => match error {
                spwn::parser::SyntaxError::ExpectedErr {
                    expected,
                    found,
                    pos,
                    file,
                } => {
                    self.client
                        .publish_diagnostics(
                            params.text_document.uri,
                            vec![Diagnostic {
                                code: None,
                                code_description: None,
                                data: None,
                                message: format!("ERROR: expected {}\nFOUND: {}", expected, found),
                                range: {
                                    let own_text = text.clone();
                                    own_text.chars();

                                    Range {
                                        start: Position {
                                            line: 1,
                                            character: 1,
                                        },
                                        end: Position {
                                            line: 1,
                                            character: 1,
                                        },
                                    }
                                },
                                severity: Some(DiagnosticSeverity::Error),
                                related_information: None,
                                source: Some("SPWN LSP syntax".to_string()),
                                tags: None,
                            }],
                            None,
                        )
                        .await;
                }
                spwn::parser::SyntaxError::UnexpectedErr { found, pos, file } => todo!(),
                spwn::parser::SyntaxError::SyntaxError { message, pos, file } => todo!(),
            },
        }
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
                        ..Default::default()
                    },
                )),

                ..Default::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::Info, "SPWN server initialized!")
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
    let start_line_number = text
        .chars()
        .take(start)
        .collect::<String>()
        .matches("\n")
        .count()
        - 1;

    let end_line_number = text
        .chars()
        .take(end)
        .collect::<String>()
        .matches("\n")
        .count()
        - 1;
}
