mod cli;
mod completion;
mod convert;
mod errors;
mod run;
mod version;

use run::*;

use std::process::*;

pub fn main() -> ExitCode {
    kutil_cli::run(run)
}
