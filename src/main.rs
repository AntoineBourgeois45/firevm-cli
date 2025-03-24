use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, name = "firevm", about = "CLI to manage microVMs", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        #[arg[short, long, default_value = "myvm.yaml"]]
        config: String,
    },
    Run {
        config: String,
    },
    Stop {
        vm_id: String,
    },
    Status {
        vm_id: String,
    },
    Snapshot {
        vm_id: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init { config } => {
            println!("Initializing VM with config: {}", config);
        }
        Commands::Run { config } => {
            println!("Running VM with config: {}", config);
        }
        Commands::Stop { vm_id } => {
            println!("Stopping VM with id: {}", vm_id);
        }
        Commands::Status { vm_id } => {
            println!("Getting status of VM with id: {}", vm_id);
        }
        Commands::Snapshot { vm_id } => {
            println!("Taking snapshot of VM with id: {}", vm_id);
        }
    }
}