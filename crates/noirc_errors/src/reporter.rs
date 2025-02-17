use crate::{FileDiagnostic, Span};
use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::term;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomDiagnostic {
    message: String,
    secondaries: Vec<CustomLabel>,
    notes: Vec<String>,
    kind: DiagnosticKind,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DiagnosticKind {
    Error,
    Warning,
}

/// A count of errors that have been already reported to stderr
#[derive(Debug, Copy, Clone)]
pub struct ReportedErrors {
    pub error_count: u32,
}

impl CustomDiagnostic {
    pub fn from_message(msg: &str) -> CustomDiagnostic {
        Self {
            message: msg.to_owned(),
            secondaries: Vec::new(),
            notes: Vec::new(),
            kind: DiagnosticKind::Error,
        }
    }

    pub fn simple_error(
        primary_message: String,
        secondary_message: String,
        secondary_span: Span,
    ) -> CustomDiagnostic {
        CustomDiagnostic {
            message: primary_message,
            secondaries: vec![CustomLabel::new(secondary_message, secondary_span)],
            notes: Vec::new(),
            kind: DiagnosticKind::Error,
        }
    }

    pub fn simple_warning(
        primary_message: String,
        secondary_message: String,
        secondary_span: Span,
    ) -> CustomDiagnostic {
        CustomDiagnostic {
            message: primary_message,
            secondaries: vec![CustomLabel::new(secondary_message, secondary_span)],
            notes: Vec::new(),
            kind: DiagnosticKind::Warning,
        }
    }

    pub fn in_file(self, file_id: fm::FileId) -> FileDiagnostic {
        FileDiagnostic { file_id, diagnostic: self }
    }

    pub fn add_note(&mut self, message: String) {
        self.notes.push(message);
    }

    pub fn add_secondary(&mut self, message: String, span: Span) {
        self.secondaries.push(CustomLabel::new(message, span));
    }

    pub fn is_error(&self) -> bool {
        matches!(self.kind, DiagnosticKind::Error)
    }
}

impl std::fmt::Display for CustomDiagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)?;

        for secondary in &self.secondaries {
            write!(f, "\nsecondary: {}", secondary.message)?;
        }

        for note in &self.notes {
            write!(f, "\nnote: {note}")?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CustomLabel {
    message: String,
    span: Span,
}

impl CustomLabel {
    fn new(message: String, span: Span) -> CustomLabel {
        CustomLabel { message, span }
    }
}

/// Writes the given diagnostics to stderr and returns the count
/// of diagnostics that were errors.
pub fn report_all(
    files: &fm::FileManager,
    diagnostics: &[FileDiagnostic],
    deny_warnings: bool,
) -> ReportedErrors {
    let error_count = diagnostics
        .iter()
        .map(|error| report(files, &error.diagnostic, Some(error.file_id), deny_warnings) as u32)
        .sum();

    ReportedErrors { error_count }
}

/// Report the given diagnostic, and return true if it was an error
pub fn report(
    files: &fm::FileManager,
    custom_diagnostic: &CustomDiagnostic,
    file: Option<fm::FileId>,
    deny_warnings: bool,
) -> bool {
    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = codespan_reporting::term::Config::default();

    let diagnostic = convert_diagnostic(custom_diagnostic, file, deny_warnings);
    term::emit(&mut writer.lock(), &config, files.as_simple_files(), &diagnostic).unwrap();

    deny_warnings || custom_diagnostic.is_error()
}

fn convert_diagnostic(
    cd: &CustomDiagnostic,
    file: Option<fm::FileId>,
    deny_warnings: bool,
) -> Diagnostic<usize> {
    let diagnostic = match (cd.kind, deny_warnings) {
        (DiagnosticKind::Warning, false) => Diagnostic::warning(),
        _ => Diagnostic::error(),
    };

    let secondary_labels = if let Some(file_id) = file {
        cd.secondaries
            .iter()
            .map(|sl| {
                let start_span = sl.span.start() as usize;
                let end_span = sl.span.end() as usize + 1;
                Label::secondary(file_id.as_usize(), start_span..end_span).with_message(&sl.message)
            })
            .collect()
    } else {
        vec![]
    };

    diagnostic.with_message(&cd.message).with_labels(secondary_labels).with_notes(cd.notes.clone())
}
