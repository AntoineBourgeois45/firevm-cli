use clap::{Parser, Subcommand};
use commands::init::execute_init;
use serde::{Serialize, Deserialize};

pub mod commands;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct VmConfig {
    pub name: String,
    pub kernel_image: String,
    pub rootfs_image: String,
    pub vcpu_count: u8,
    pub memory_mb: u32,
    pub network: NetworkConfig,
    pub snapshot: SnapshotConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub mode: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotConfig {
    pub enabled: bool,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init { config } => {
            execute_init(config);
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