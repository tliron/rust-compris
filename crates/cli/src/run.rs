use super::{cli::*, completion::*, errors::*, version::*};

use clap::*;

pub fn run() -> Result<(), MainError> {
    let cli = CLI::parse();

    if !cli.quiet {
        cli.output_colorize.apply();
        kutil_cli::tracing(cli.verbose + 2);
    }

    match &cli.subcommand {
        None => cli.convert()?,
        Some(subcommand) => match subcommand {
            SubCommand::Version => version(),
            SubCommand::Completion { shell } => completion(shell),
        },
    }

    Ok(())
}
