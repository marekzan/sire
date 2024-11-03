mod commands;

use core::{db::migration::start_migration, io::data::load_data};
use std::{env, error::Error};

use clap::{command, Parser, Subcommand};
use commands::{
    add::{add, add_db},
    check::{check, check_db},
    remove::remove,
};

#[derive(Parser)]
#[command(name = "sire")]
#[command(bin_name = "sire")]
#[command(about = "A simple reminder")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
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
    Dbadd {
        #[arg(short, long)]
        name: String,

        #[arg(short, long)]
        age: i32,

        #[arg(short, long)]
        email: String,
    },
    Dbcheck {},
}

fn main() -> Result<(), Box<dyn Error>> {
    start_migration()?;
    let path = env::current_dir()?;
    let path = match path.to_str() {
        Some(path) => path,
        None => panic!("Could not find current dir. Exiting!"),
    };

    let mut data = load_data()?;
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Check {}) => check(path, &data)?,
        Some(Commands::Add { content, detached }) => {
            add(path, &mut data, content.clone(), detached)?
        }
        Some(Commands::Remove {}) => remove(),
        Some(Commands::Dbadd { name, age, email }) => add_db(name, age, email)?,
        Some(Commands::Dbcheck {}) => check_db()?,
        None => println!("No command provided"),
    }
    Ok(())
}
