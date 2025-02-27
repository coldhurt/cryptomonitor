use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "test_clap_parser")]
#[command(version = "1.0")]
#[command(author = "myt0.com")]
#[command(about = "Just an example for clap parser", long_about = None)]
struct Cli {
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
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },

    /// do file things
    File {
        /// Input file
        #[arg(long, short)]
        input: String,

        /// Output file
        #[arg(long, short)]
        output: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.config {
        Some(config) => {
            println!("config: {:?}", config);
        }
        None => {}
    }

    println!("debug: {:?}", cli.debug);

    match cli.command {
        Some(Commands::Test { list }) => {
            println!("list: {:?}", list);
        }
        Some(Commands::File { input, output }) => {
            println!("input: {:?}, output: {:?}", input, output);
        }
        None => {}
    }
}
