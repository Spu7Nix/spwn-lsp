use std::path::PathBuf;

use lsp_types::{Diagnostic, DiagnosticSeverity, Url};
use spwn::parse_spwn;

use crate::utils::compute_range;

pub async fn set_syntax_errors(text: &String, text_location: Url) -> Vec<Diagnostic> {
    let parsed = parse_spwn(text.clone(), PathBuf::from(text_location.path()));
    let mut diagnostics = Vec::<Diagnostic>::new();

    if let Err(error) = parsed {
        match error {
            spwn::parser::SyntaxError::ExpectedErr {
                expected,
                found,
                pos,
                file: _,
            } => diagnostics.push(Diagnostic {
                code: None,
                code_description: None,
                data: None,
                message: format!("ERROR: expected {},\nFOUND: {}", expected, found),
                range: compute_range(text.replace("\r\n", "\n"), pos),
                severity: Some(DiagnosticSeverity::Error),
                related_information: None,
                source: Some("SPWN Syntax Error (Expected)".to_string()),
                tags: None,
            }),

            spwn::parser::SyntaxError::UnexpectedErr { found, pos, .. } => {
                diagnostics.push(Diagnostic {
                    code: None,
                    code_description: None,
                    data: None,
                    message: format!("ERROR: unexpected {}", found),
                    range: compute_range(text.replace("\r\n", "\n"), pos),
                    severity: Some(DiagnosticSeverity::Error),
                    related_information: None,
                    source: Some("SPWN Syntax Error (Unexpected)".to_string()),
                    tags: None,
                })
            }
            spwn::parser::SyntaxError::SyntaxError { message, pos, .. } => {
                diagnostics.push(Diagnostic {
                    code: None,
                    code_description: None,
                    data: None,
                    message: format!("SYNTAX ERROR: {}", message),
                    range: compute_range(text.replace("\r\n", "\n"), pos),
                    severity: Some(DiagnosticSeverity::Error),
                    related_information: None,
                    source: Some("SPWN Syntax Error".to_string()),
                    tags: None,
                })
            }
        }
    };

    diagnostics
}
