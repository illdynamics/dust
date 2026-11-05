//! Structured diagnostics for the Dust compiler.
//!
//! A [`Diagnostic`] carries a severity, an optional stable error code, one or
//! more labeled spans, and free-form notes. Diagnostics render to any writer
//! with source snippets and line/column numbers so they are deterministic and
//! unit-testable without touching the terminal.

use crate::source::{SourceFile, Span};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
}

impl Default for Severity {
    /// Errors are the conservative default severity.
    fn default() -> Self {
        Severity::Error
    }
}

impl Severity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Error => "error",
            Severity::Warning => "warning",
        }
    }
}

/// A labelled region of source text. `primary` controls emphasis in output.
#[derive(Debug, Clone)]
pub struct Label {
    pub span: Span,
    pub message: String,
    pub primary: bool,
}

impl Label {
    pub fn new(span: Span, message: impl Into<String>) -> Self {
        Label { span, message: message.into(), primary: false }
    }
    pub fn primary(mut self) -> Self {
        self.primary = true;
        self
    }
}

#[derive(Debug, Clone, Default)]
pub struct Diagnostic {
    pub severity: Severity,
    pub code: Option<String>,
    pub message: String,
    pub labels: Vec<Label>,
    pub notes: Vec<String>,
}

impl Diagnostic {
    pub fn new(severity: Severity, message: impl Into<String>) -> Self {
        Diagnostic { severity, code: None, message: message.into(), labels: vec![], notes: vec![] }
    }
    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }
    pub fn label(mut self, span: Span, message: impl Into<String>) -> Self {
        self.labels.push(Label::new(span, message));
        self
    }
    pub fn primary_label(mut self, span: Span, message: impl Into<String>) -> Self {
        let mut l = Label::new(span, message);
        l.primary = true;
        self.labels.push(l);
        self
    }
    pub fn note(mut self, text: impl Into<String>) -> Self {
        self.notes.push(text.into());
        self
    }

    fn header(&self) -> String {
        match &self.code {
            Some(code) => format!("{}[{}] {}", self.severity.as_str(), code, self.message),
            None => format!("{}: {}", self.severity.as_str(), self.message),
        }
    }

    /// Render this diagnostic against the given sources into a String.
    pub fn render(&self, sources: &[&SourceFile]) -> String {
        let mut out = String::new();
        out.push_str(&format!("{}\n", self.header()));

        for label in &self.labels {
            let span = label.span;
            let (where_, snippet) = locate(sources, span);
            if label.primary {
                out.push_str(&format!("  --> {}\n", where_));
            } else {
                out.push_str(&format!("  = note: {} at {}\n", label.message, where_));
            }
            if let Some((line_text, col_start, col_end)) = snippet {
                out.push_str(&format!("    | {}\n", line_text));
                if col_end > col_start {
                    let pad = " ".repeat(col_start.saturating_sub(1) as usize);
                    let under = "~".repeat((col_end - col_start) as usize);
                    out.push_str(&format!("    | {}{}\n", pad, under));
                }
            }
            if label.primary && !label.message.is_empty() {
                out.push_str(&format!("    = {}: {}\n", "label", label.message));
            }
        }

        for note in &self.notes {
            out.push_str(&format!("  = note: {}\n", note));
        }
        out
    }
}

/// Locate a span and, if possible, render its source line plus underline columns.
fn locate<'a>(sources: &'a [&SourceFile], span: Span) -> (String, Option<(String, u32, u32)>) {
    for src in sources {
        let n = src.bytes().len() as u32;
        if span.start < n {
            let loc = src.offset_to_location(span.start);
            let (line_text, marker_opt) = src.render_snippet(span);
            let where_ = format!("{}:{}:{}", loc.file, loc.line, loc.col);
            // Re-wrap into the (text, col_start, col_end) tuple shape.
            let marker = match (marker_opt, line_text) {
                (Some((cs, ce)), t) => Some((t, cs, ce)),
                (None, _) => None,
            };
            return (where_, marker);
        }
    }
    ("<unknown location>".to_string(), None)
}
