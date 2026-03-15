use clap::{Parser, Subcommand};
use kvs::KvStore;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>
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
    },
}


fn main() {
    let args = Args::parse();
    let parsed = args.command.unwrap();
    let mut store = KvStore::new();

    match parsed {
        Commands::Set{key, value} => {
            store.set(key, value);
        }
        Commands::Get{key} => {
            let got = store.get(key);
        }
        Commands::Rm{key} => {
            store.remove(key);
        }
    }
}