use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long)]
    debug: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add user
    Add { name: Option<String> },
}

fn main() {
    let cli = Cli::parse();

    match cli.debug {
        true => println!("Debugging on"),
        false => println!("Debugging off"),
    }

    match &cli.command {
        Commands::Add {name} => {
            println!("Adding user: {name:?}");
            if let Some(name) = name {
                match name.as_str() {
                    "bob" => println!("!"),
                    _ => println!("Oh! Hey YOU!!!! (obviously doesn't know your name)"),
                }
            }
        }
    }
}