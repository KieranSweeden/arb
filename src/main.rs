use clap::{Args, Parser, Subcommand, ValueEnum};
use uuid::Uuid;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate arbitrary data
    Gen(GenerateArgs),
}

#[derive(Args, Debug)]
struct GenerateArgs {
    #[command(subcommand)]
    data_type: DataTypes,
}

#[derive(Subcommand, Clone, Debug)]
enum DataTypes {
    /// Generate an arbitrary uuid
    Uuid(UuidArgs),
}

#[derive(Args, Debug, Clone)]
struct UuidArgs {
    /// Select version
    version: UuidVersions,
}

#[derive(ValueEnum, Clone, Debug)]
enum UuidVersions {
    /// Generate an arbitrary uuid v4
    V4,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Gen(generate_args) => match &generate_args.data_type {
            DataTypes::Uuid(uuid_args) => match &uuid_args.version {
                UuidVersions::V4 => {
                    let uuid = Uuid::new_v4();
                    println!("{uuid}");
                }
            },
        },
    }
}
