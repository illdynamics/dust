//! Compiler driver orchestration (loading, compilation, diagnostic emission).

use crate::diagnostics::Diagnostic;
use crate::source::SourceFile;
use std::io::{self, Write};

/// The compiler's in-memory "source map": one entry per loaded translation unit.
pub struct SourceMap {
    files: Vec<SourceFile>,
}

impl SourceMap {
    pub fn new() -> Self {
        SourceMap { files: Vec::new() }
    }
    pub fn add(&mut self, mut sf: SourceFile) -> u32 {
        let id = self.files.len() as u32;
        sf.set_id(id);
        self.files.push(sf);
        id
    }
    pub fn borrow(&self) -> Vec<&SourceFile> {
        self.files.iter().collect()
    }
}

/// Compile the given sources. Returns diagnostics to report (empty on success).
pub fn compile(sources: &[&SourceFile]) -> Vec<Diagnostic> {
    // Scaffold stage: no front end yet, so every loaded source compiles cleanly.
    let _ = sources;
    Vec::new()
}

/// Render diagnostics to `out`.
pub fn emit_diagnostics(
    out: &mut dyn Write,
    sources: &[&SourceFile],
    diags: &[Diagnostic],
) -> io::Result<()> {
    for d in diags {
        let rendered = d.render(sources);
        out.write_all(rendered.as_bytes())?;
        out.write_all(b"\n")?;
    }
    Ok(())
}

/// Run the driver given a pre-parsed CLI. Returns an error string on failure.
pub fn run(cli: crate::cli::Cli) -> Result<(), String> {
    let input = match &cli.input {
        Some(sf) => sf.name().to_string(),
        None => return Err("no input file given".to_string()),
    };

    let mut smap = SourceMap::new();
    smap.add(cli.input.clone().unwrap());
    let refs = smap.borrow();
    let diags = compile(&refs);

    if !diags.is_empty() {
        emit_diagnostics(&mut io::stderr(), &refs, &diags)
            .map_err(|e| format!("failed to write diagnostics: {}", e))?;
        return Err(format!("{} diagnostic(s) emitted while compiling '{}'", diags.len(), input));
    }

    println!("compiled {} (0 diagnostics)", input);
    Ok(())
}
