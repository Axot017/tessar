use build_lambda::build_lambda;
use clap::Parser;
use command::{Cli, Command};
use error::DynError;

mod build_lambda;
mod cargo_content;
mod command;
mod error;
mod utils;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(-1);
    }
}

fn try_main() -> Result<(), DynError> {
    let cli = Cli::parse();

    match &cli.command {
        Command::BuildLambda(args) => build_lambda(args),
    }
}
