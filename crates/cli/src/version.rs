use super::cli::*;

use clap::*;

pub fn version() {
    print!("{}", CLI::command().render_version());
}
