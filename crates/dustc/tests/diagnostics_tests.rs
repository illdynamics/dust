//! Tests for structured diagnostics rendering (S02-T04).

use dustc::diagnostics::{Diagnostic, Severity};
use dustc::source::{SourceFile, Span};

fn diag_with_source() -> (Diagnostic, SourceFile) {
    let s = SourceFile::from_str("main.dust", "let x = 1;\n".to_string());
    let d = Diagnostic::new(Severity::Error, "cannot assign to immutable variable")
        .with_code("E0277")
        .primary_label(Span::new(4, 5), "`x` is immutable")
        .note("consider using `var x` for a mutable binding");
    (d, s)
}

#[test]
fn render_includes_severity_code_and_message() {
    let (d, src) = diag_with_source();
    let rendered = d.render(&[&src]);
    assert!(
        rendered.contains("error[E0277] cannot assign to immutable variable"),
        "got: {rendered}"
    );
}

#[test]
fn render_includes_location_and_snippet() {
    let (d, src) = diag_with_source();
    let rendered = d.render(&[&src]);
    assert!(rendered.contains("main.dust:1:5"), "location missing:\n{rendered}");
    assert!(rendered.contains("let x = 1;"), "snippet missing:\n{rendered}");
    // Underline of tildes under the offending token.
    assert!(rendered.contains("~"), "underline missing:\n{rendered}");
    // The primary label message and note both appear.
    assert!(rendered.contains("`x` is immutable"));
    assert!(rendered.contains("consider using `var x`"));
}

#[test]
fn warning_severity_string() {
    let d = Diagnostic::new(Severity::Warning, "unused variable")
        .primary_label(Span::point(0), "warning here");
    let rendered = d.render(&[]);
    assert!(rendered.contains("warning: unused variable"));
}
