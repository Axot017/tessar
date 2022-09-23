use clap::{Args, Parser, Subcommand};

pub mod build_lambda;
pub mod create_dart_layer;
pub mod create_flutter_layer;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    BuildLambda(BuildLambdaArgs),
    CreateFlutterLayer(CreateFlutterLayerArgs),
    CreateDartLayer(CreateDartLayerArgs),
}

#[derive(Args, Debug)]
pub struct BuildLambdaArgs {
    #[clap(short, long, default_value_t = String::from("aarch64-unknown-linux-gnu"))]
    pub target: String,
    #[clap(short, long, default_value_t = String::from("bootstrap"))]
    pub entrypoint: String,
    #[clap(short, long)]
    pub use_cross: bool,
}

#[derive(Args, Debug)]
pub struct CreateFlutterLayerArgs {}

#[derive(Args, Debug)]
pub struct CreateDartLayerArgs {
    #[clap(short, long)]
    pub version: String,
}
