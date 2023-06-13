use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::{bake_init, helpers::check_for_bake_config};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// New project dialog
    Init {
        /// lists values
        #[arg(short, long)]
        list: bool,
    },

    /// Just here for testing
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
}

pub fn clap_cli(cli: Cli) {
    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => {}
        1 => println!("'d' flag counted once"),
        2 => println!("'d' flag counted twice"),
        _ => println!("'d' flag counted more then twice"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Test { list }) => {
            if *list {
                println!("Printing testing lists...");
            }
        }
        Some(Commands::Init { list }) => {
            bake_init::initialize_project();
        }
        None => {
            if !check_for_bake_config() {
                println!("No Bake.toml found");
            } else {
                println!("Your project is baking...");
            }
        }
    }
}
