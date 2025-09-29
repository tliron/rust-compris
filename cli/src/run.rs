use super::{cli::*, errors::*};

use {clap::*, kutil::cli::log::*};

pub fn run() -> Result<(), MainError> {
    let cli = CLI::parse();

    if !cli.quiet {
        cli.output_colorize.initialize();
        initialize_tracing(cli.verbose + 2, cli.log_path.as_ref())?;
    }

    match &cli.subcommand {
        None => cli.convert()?,
        Some(SubCommand::Version(version)) => version.run::<CLI>(),
        Some(SubCommand::Completion(completion)) => completion.run::<CLI>(),
        Some(SubCommand::Manual(manual)) => manual.run::<CLI>()?,
    }

    Ok(())
}
