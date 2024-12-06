use super::cli::*;

use clap::*;

pub(crate) fn version() {
    print!("{}", CLI::command().render_version());
}
