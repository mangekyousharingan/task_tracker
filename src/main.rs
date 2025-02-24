use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
enum Action {
    Add {data:String},
    Update{id:i32, updated_data: String},
    Delete{id:i32},
    List,
}

fn main() {
    let cli = Cli::parse();
    match cli.action {
        Action::Add { data } => println!("Adding task: {data}"),
        Action::Update { id, updated_data } => println!("Updating data: {id} with {updated_data}"),
        Action::Delete { id } => println!("Deleting data: {id}"),
        Action::List => println!("Listing all data!"),
    }
}
