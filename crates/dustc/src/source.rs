//! Source-file model for Dust.
//!
//! A single [`SourceFile`] owns one `.dust` translation unit: its name, bytes,
//! a stable file id, and precomputed line-start offsets so spans can be turned
//! into (line, column) pairs and rendered with source snippets on demand.

/// Byte offset within a [`SourceFile`].
pub type Offset = u32;

/// A half-open character span `[start, end)` in bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: Offset,
    pub end: Offset,
}

impl Span {
    /// A half-open byte span `[start, end)`.
    pub const fn new(start: Offset, end: Offset) -> Self {
        Span { start, end }
    }

    /// An empty span at a single byte offset (useful for point-like labels).
    pub const fn point(offset: Offset) -> Self {
        Span { start: offset, end: offset }
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

/// A 1-based line number.
pub type LineNo = u32;
/// A 1-based column number (UTF-8 byte column).
pub type ColNo = u32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileLocation {
    pub file: String,
    pub line: LineNo,
    pub col: ColNo,
}

/// One loaded Dust source file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceFile {
    id: u32,
    name: String,
    bytes: Vec<u8>,
    /// Byte offset of the first character on each line (line 1 => [0]).
    line_starts: Vec<Offset>,
}

impl SourceFile {
    /// Load a source file from disk.
    pub fn load(path: &str) -> Result<Self, String> {
        match std::fs::read_to_string(path) {
            Ok(contents) => Ok(Self::from_str(path, contents)),
            Err(e) => Err(format!("cannot read '{}': {}", path, e)),
        }
    }

    /// Build a source file in memory from its name and UTF-8 contents.
    pub fn from_str(name: &str, contents: String) -> Self {
        let bytes = contents.into_bytes();
        // line_starts[0] == 0 always; each subsequent entry is the offset right
        // after a '\n'. We scan once so span->location conversion is O(log n).
        let mut line_starts = Vec::with_capacity(1);
        line_starts.push(0u32);
        for (i, &b) in bytes.iter().enumerate() {
            if b == b'\n' {
                // next line starts one byte after the newline.
                line_starts.push((i + 1) as Offset);
            }
        }
        SourceFile { id: 0, name: name.to_string(), bytes, line_starts }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn set_id(&mut self, id: u32) {
        self.id = id;
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Convert a byte offset into a 1-based (line, column).
    pub fn offset_to_location(&self, offset: Offset) -> FileLocation {
        let offset = offset.min(self.line_starts.last().copied().unwrap_or(0));
        // Binary search for the line whose start is <= offset.
        let line_idx = match self.line_starts.binary_search(&offset) {
            Ok(i) => i,
            Err(next) => next - 1,
        };
        let line = line_idx as LineNo + 1;
        let start_of_line = self.line_starts[line_idx];
        // Column is a 1-based byte column.
        let col = (offset - start_of_line) as ColNo + 1;
        FileLocation { file: self.name.clone(), line, col }
    }

    /// Render the text of a span. Empty spans yield the single character at the
    /// offset if any exists on that line.
    pub fn span_text(&self, span: Span) -> String {
        let end = span.end.min(self.bytes.len() as Offset);
        let start = span.start.min(end);
        if start >= end {
            // Point-like; return the single byte at `start` when on the line.
            if (start as usize) < self.bytes.len() {
                let b = self.bytes[start as usize];
                if b == b'\n' {
                    String::new()
                } else {
                    char::from_u32(b as u32).map_or_else(|| "?".to_string(), |c| c.to_string())
                }
            } else {
                String::new()
            }
        } else {
            String::from_utf8_lossy(&self.bytes[start as usize..end as usize]).into_owned()
        }
    }

    /// Render a snippet: the line containing `offset` plus markers.
    pub fn render_snippet(&self, span: Span) -> (String, Option<(ColNo, ColNo)>) {
        let loc = self.offset_to_location(span.start);
        let line_idx = (loc.line as usize).saturating_sub(1);
        if line_idx >= self.line_starts.len() {
            return (String::new(), None);
        }
        let start_line = self.line_starts[line_idx];
        let end_line = if line_idx + 1 < self.line_starts.len() {
            self.line_starts[line_idx + 1]
        } else {
            self.bytes.len() as Offset
        };
        // Trim trailing '\r' for CRLF files.
        let mut line_bytes = &self.bytes[start_line as usize..end_line as usize];
        if line_bytes.last() == Some(&b'\r') {
            line_bytes = &line_bytes[..line_bytes.len() - 1];
        }
        let text = String::from_utf8_lossy(line_bytes).into_owned();
        // Underline spans the bytes of `span` on this line.
        let col_start = loc.col.max(1);
        let span_len = (span.end - span.start).min(text.len() as u32).max(1);
        let col_end = (col_start + span_len).max(col_start + 1);
        let marker = Some((col_start, col_end));
        (text, marker)
    }
}
