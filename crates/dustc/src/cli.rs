//! Command-line interface for `dustc`.
//!
//! Argument parsing is hand-rolled (no external crate) to keep the dependency
//! footprint minimal and the surface fully documented.

use crate::source::SourceFile;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Emit {
    /// Emit C11 source alongside/instead of the native object.
    C,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cli {
    pub version: bool,
    pub help: bool,
    pub no_color: bool,
    pub emit: Option<Emit>,
    pub output: Option<String>,
    pub input: Option<SourceFile>,
}

fn parse_emit(value: &str) -> Result<Emit, String> {
    match value {
        "c" => Ok(Emit::C),
        other => {
            Err(format!("unknown emit backend '{}' (only 'c' is supported in this release)", other))
        }
    }
}

/// Parse argv (excluding the program name). Returns Ok(Cli) or Err(message).
pub fn parse<I: Iterator<Item = String>>(mut args: I) -> Result<Cli, String> {
    let mut cli =
        Cli { version: false, help: false, no_color: false, emit: None, output: None, input: None };
    let mut positionals: Vec<String> = Vec::new();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--version" | "-V" => cli.version = true,
            "--help" | "-h" => cli.help = true,
            "--no-colors" | "--nocolor" => cli.no_color = true,
            "-o" | "--output" => {
                let v = args.next().ok_or_else(|| format!("missing argument for {}", arg))?;
                cli.output = Some(v);
            }
            "--emit" => {
                let v = args.next().ok_or_else(|| "missing argument for --emit".to_string())?;
                cli.emit = Some(parse_emit(&v)?);
            }
            s if s.starts_with("--emit=") => {
                cli.emit = Some(parse_emit(&s["--emit=".len()..])?);
            }
            s if s.starts_with("-") && s != "-" => {
                return Err(format!("unknown flag '{}'", s));
            }
            _ => positionals.push(arg),
        }
    }

    // Load the single positional input file, if any.
    match positionals.as_slice() {
        [] => Ok(cli),
        [path] => match SourceFile::load(path) {
            Ok(sf) => {
                cli.input = Some(sf);
                Ok(cli)
            }
            Err(e) => Err(e),
        },
        [_, extra, ..] => Err(format!(
            "unexpected extra input file '{}' (only one INPUT.dust is accepted)",
            extra
        )),
    }
}

/// Human-readable usage text (also used for --help).
pub fn usage(program: &str) -> String {
    let mut s = String::new();
    s.push_str("Dust compiler — native systems language for constrained devices.\n");
    s.push_str("\nUSAGE:\n    ");
    s.push_str(program);
    s.push_str(" [OPTIONS] <INPUT.dust>\n");
    s.push_str("\nOPTIONS:\n");
    s.push_str("  -V, --version           Print version information and exit.\n");
    s.push_str("  -h, --help              Print this help and exit.\n");
    s.push_str("      --emit=BACKEND      Emit backend (only 'c' is supported).\n");
    s.push_str("  -o, --output <PATH>     Output path for generated artifacts.\n");
    s.push_str("      --no-colors         Disable colored diagnostics.\n");
    s.push_str("\nEXAMPLES:\n    ");
    s.push_str(program);
    s.push_str(" hello.dust\n    ");
    s.push_str(program);
    s.push_str(" --emit=c -o out.c hello.dust");
    s
}
