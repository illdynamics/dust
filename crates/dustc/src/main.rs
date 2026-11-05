//! `dustc` binary entry point.

use std::process::ExitCode;

fn main() -> ExitCode {
    let args = std::env::args().skip(1);
    let program = std::env::args().next().unwrap_or_else(|| "dustc".to_string());
    match dustc::cli::parse(args) {
        Err(e) => {
            eprintln!("error: {}", e);
            ExitCode::FAILURE
        }
        Ok(cli) if cli.version => {
            println!("{} {} {}", program, "dustc", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        Ok(cli) if cli.help => {
            print!("{}", dustc::cli::usage(&program));
            ExitCode::SUCCESS
        }
        Ok(cli) => match dustc::driver::run(cli) {
            Ok(()) => ExitCode::SUCCESS,
            Err(_) => ExitCode::FAILURE,
        },
    }
}
