mod cli;
mod convert;
mod errors;
mod run;

use run::*;

use std::process::*;

pub fn main() -> ExitCode {
    kutil_cli::run::run(run)
}
