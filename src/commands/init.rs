use crate::{NetworkConfig, SnapshotConfig, VmConfig};

pub fn execute_init(config_path: &str) {
    let default_config = VmConfig {
        name: "myvm".to_string(),
        kernel_image: "./vmlinux".to_string(),
        rootfs_image: "./rootfs.ext4".to_string(),
        vcpu_count: 1,
        memory_mb: 256,
        network: NetworkConfig {
            mode: "nat".to_string(),
        },
        snapshot: SnapshotConfig {
            enabled: false,
        },
    };

    match serde_yaml::to_string(&default_config) {
        Ok(yaml_string) => {
            match std::fs::write(config_path, yaml_string) {
                Ok(_) => println!("Config file created: {}", config_path),
                Err(e) => eprintln!("Error when writing on file: {}", e),
            }
        }
        Err(e) => eprintln!("Error while serializing config: {}", e),
    }
}
