use lsp_types::*;
use taplo::{dom, parser::Parse, util::coords::Mapper};

pub fn collect_diagnostics(uri: &Url, parse: &Parse, mapper: &Mapper) -> Vec<Diagnostic> {
    let mut diag: Vec<Diagnostic> = parse
        .errors
        .iter()
        .filter_map(|e| {
            let range = mapper.range(e.range).unwrap();
            Some(Diagnostic {
                range,
                severity: Some(DiagnosticSeverity::Error),
                code: None,
                source: Some("Even Better TOML".into()),
                message: e.message.clone().into(),
                related_information: None,
                tags: None,
            })
        })
        .collect();

    for err in parse.clone().into_dom().errors() {
        match err {
            dom::Error::DuplicateKey { first, second } => {
                let first_range = mapper.range(first.text_range()).unwrap();
                let second_range = mapper.range(second.text_range()).unwrap();

                diag.push(Diagnostic {
                    range: first_range,
                    severity: Some(DiagnosticSeverity::Error),
                    code: None,
                    source: Some("Even Better TOML".into()),
                    message: format!(r#"duplicate key "{}""#, first.full_key()),
                    related_information: Some(vec![DiagnosticRelatedInformation {
                        location: Location {
                            range: second_range,
                            uri: uri.clone(),
                        },
                        message: "other declaration".into(),
                    }]),
                    tags: None,
                });

                diag.push(Diagnostic {
                    range: second_range,
                    severity: Some(DiagnosticSeverity::Error),
                    code: None,
                    source: Some("Even Better TOML".into()),
                    message: format!(r#"duplicate key "{}""#, first.full_key()),
                    related_information: Some(vec![DiagnosticRelatedInformation {
                        location: Location {
                            range: first_range,
                            uri: uri.clone(),
                        },
                        message: "first declaration".into(),
                    }]),
                    tags: None,
                });
            }
            dom::Error::ExpectedTable { target, key } => {
                let target_range = mapper.range(target.text_range()).unwrap();
                let second_range = mapper.range(key.text_range()).unwrap();

                diag.push(Diagnostic {
                    range: target_range,
                    severity: Some(DiagnosticSeverity::Error),
                    code: None,
                    source: Some("Even Better TOML".into()),
                    message: format!(r#"expected table for "{}""#, target.full_key()),
                    related_information: Some(vec![DiagnosticRelatedInformation {
                        location: Location {
                            range: second_range,
                            uri: uri.clone(),
                        },
                        message: format!(r#"required by "{}""#, key.full_key()),
                    }]),
                    tags: None,
                });
            }
            dom::Error::ExpectedTableArray { target, key } => {
                let target_range = mapper.range(target.text_range()).unwrap();
                let second_range = mapper.range(key.text_range()).unwrap();

                diag.push(Diagnostic {
                    range: target_range,
                    severity: Some(DiagnosticSeverity::Error),
                    code: None,
                    source: Some("Even Better TOML".into()),
                    message: format!(r#"array conflicts with array of tables"#),
                    related_information: Some(vec![DiagnosticRelatedInformation {
                        location: Location {
                            range: second_range,
                            uri: uri.clone(),
                        },
                        message: format!(r#"array of tables declaration"#),
                    }]),
                    tags: None,
                });

                diag.push(Diagnostic {
                    range: second_range,
                    severity: Some(DiagnosticSeverity::Error),
                    code: None,
                    source: Some("Even Better TOML".into()),
                    message: format!(r#"array conflicts with array of tables"#),
                    related_information: Some(vec![DiagnosticRelatedInformation {
                        location: Location {
                            range: target_range,
                            uri: uri.clone(),
                        },
                        message: format!(r#"array declaration"#),
                    }]),
                    tags: None,
                });
            }
            dom::Error::InlineTable { target, key } => {
                let target_range = mapper.range(target.text_range()).unwrap();
                let second_range = mapper.range(key.text_range()).unwrap();

                diag.push(Diagnostic {
                    range: target_range,
                    severity: Some(DiagnosticSeverity::Error),
                    code: None,
                    source: Some("Even Better TOML".into()),
                    message: format!(r#"inline table cannot be modified"#),
                    related_information: Some(vec![DiagnosticRelatedInformation {
                        location: Location {
                            range: second_range,
                            uri: uri.clone(),
                        },
                        message: format!(r#"modified here by "{}""#, key.full_key()),
                    }]),
                    tags: None,
                });

                diag.push(Diagnostic {
                    range: second_range,
                    severity: Some(DiagnosticSeverity::Error),
                    code: None,
                    source: Some("Even Better TOML".into()),
                    message: format!(r#"inline table cannot be modified"#),
                    related_information: Some(vec![DiagnosticRelatedInformation {
                        location: Location {
                            range: target_range,
                            uri: uri.clone(),
                        },
                        message: format!(r#"inline table "{}" here"#, key.full_key()),
                    }]),
                    tags: None,
                });
            }
            dom::Error::Spanned { range, message } => {
                let r = mapper.range(range.clone()).unwrap();

                diag.push(Diagnostic {
                    range: r,
                    severity: Some(DiagnosticSeverity::Error),
                    code: None,
                    source: Some("Even Better TOML".into()),
                    message: message.clone(),
                    related_information: None,
                    tags: None,
                });
            }
            dom::Error::Generic(_) => {
                // todo show this as well somewhere?
            }
        }
    }

    diag
}
