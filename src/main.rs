use clap::{Parser, Subcommand};
use ledger;
use transactions::list_all_transactions;
mod transactions;

#[derive(Parser)]
#[command(author, version, about, long_about)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init {},
    Import { filename: String },
    List {},
    Test {},
}

fn main() {
    let cli = Args::parse();

    match &cli.command {
        Commands::Init {  } => ledger::init(),
        Commands::Import { filename } => transactions::import_transactions(filename),
        Commands::List {  } => list_all_transactions(),
        Commands::Test {  } => ledger::create_transaction(),
        _ => println!("Fall through"),
    }
}
