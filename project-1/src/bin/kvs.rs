use clap::{Parser, Subcommand};
use kvs::KvStore;

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
#[derive(Debug)]
enum Commands {
    Set {
        key: String,
        value: String,
    },
    Get {
        key: String
    },
    Rm {
        key: String
    }
}


fn main() {
    let args = Args::parse();
    let parsed = args.command.unwrap();

    match parsed {
        Commands::Set{key, value} => {
            println!("unimplemented")
        }
        Commands::Get{key} => {
            println!("unimplemented")
        }
        Commands::Rm{key} => {
            println!("unimplemented")
        }
    }
}