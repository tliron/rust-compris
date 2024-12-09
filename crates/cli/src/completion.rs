use super::cli::*;

use {clap::*, clap_complete_command::*, std::io};

pub fn completion(shell: &Shell) {
    shell.generate(&mut CLI::command(), &mut io::stdout());
}
