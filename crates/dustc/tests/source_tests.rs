//! Positive/negative tests for the source-file model (S02-T03).

use dustc::source::{FileLocation, SourceFile, Span};

fn src(contents: &str) -> SourceFile {
    SourceFile::from_str("test.dust", contents.to_string())
}

#[test]
fn offset_to_location_first_line() {
    let s = src("abc\ndef\n");
    assert_eq!(s.offset_to_location(0), FileLocation { file: "test.dust".into(), line: 1, col: 1 });
    assert_eq!(s.offset_to_location(2), FileLocation { file: "test.dust".into(), line: 1, col: 3 });
    // 'c'
}

#[test]
fn offset_to_location_second_line() {
    let s = src("abc\ndef\n");
    // offset 4 is 'd' on line 2, column 1.
    assert_eq!(s.offset_to_location(4), FileLocation { file: "test.dust".into(), line: 2, col: 1 });
    assert_eq!(s.offset_to_location(6), FileLocation { file: "test.dust".into(), line: 2, col: 3 });
    // 'f'
}

#[test]
fn offset_clamps_at_end() {
    let s = src("ab\n");
    // Offset past end clamps to last known column.
    let loc = s.offset_to_location(100);
    assert!(loc.line >= 1 && loc.col >= 1);
}

#[test]
fn span_text_returns_substring() {
    let s = src("let x = 42;\n");
    // "x" is at bytes [4,5); "let" at [0,3).
    assert_eq!(s.span_text(Span::new(4, 5)), "x");
    assert_eq!(s.span_text(Span::new(0, 3)), "let");
}

#[test]
fn span_text_handles_utf8_multibyte() {
    let s = src("aé\n"); // 'é' is 2 bytes at offset 1..3
    assert_eq!(s.span_text(Span::new(1, 3)), "é");
    let loc = s.offset_to_location(1);
    assert_eq!((loc.line, loc.col), (1, 2));
}

#[test]
fn render_snippet_provides_underline() {
    let s = src("let x = 42;\n");
    let (text, marker) = s.render_snippet(Span::new(4, 5)); // the 'x'
    assert_eq!(text.trim(), "let x = 42;");
    let (cs, ce) = marker.expect("underline present");
    // 'x' is column 5..6 in "let x = 42;"
    assert_eq!((cs, ce), (5, 6));
}

#[test]
fn render_snippet_trims_carriage_return() {
    let s = src("line\r\nsecond\n");
    let (text, _) = s.render_snippet(Span::point(0));
    assert!(!text.ends_with('\r'), "CRLF should be trimmed: {:?}", text);
}

#[test]
fn span_constructors_and_predicates() {
    let p = Span::point(5);
    assert!(p.is_empty());
    assert_eq!((p.start, p.end), (5, 5));
}
