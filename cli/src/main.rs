mod commands;

use core::io::data::load_data;
use std::env;

use clap::{command, Parser, Subcommand};
use commands::{add::add, check::check, remove::remove};

#[derive(Parser)]
#[command(name = "sire")]
#[command(bin_name = "sire")]
#[command(about = "A simple reminder")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Check {},
    Add {
        #[arg(short, long)]
        content: String,

        #[arg(short, long)]
        detached: bool,
    },
    Remove {},
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::current_dir()?;
    let path = match path.to_str() {
        Some(path) => path,
        None => panic!("Could not find current dir. Exiting!"),
    };

    let mut data = load_data()?;
    let cli = Cli::parse();

    match &cli.command {
        Commands::Check {} => check(path, &data)?,
        Commands::Add { content, detached } => add(path, &mut data, content.clone(), detached)?,
        Commands::Remove {} => remove(),
    }
    Ok(())
}
