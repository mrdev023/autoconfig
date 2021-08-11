use std::io::{Write, Read};

use super::ConfigMode;
use super::super::super::file_utils;

pub fn configure_env(mode: &ConfigMode) -> Option<()> {
    let binary_folder_path = file_utils::get_install_dir(&file_utils::InstallType::Command).ok()?;
    let binary_folder_str = binary_folder_path.to_str()?;
    let bash_file = dirs::home_dir()?.join(".bashrc");
    let env_path_str = format!("export PATH=\"$PATH:{}\"\n", binary_folder_str);
    
    let mut file = std::fs::OpenOptions::new()
        .create_new(!bash_file.exists())
        .write(true)
        .read(true)
        .append(true)
        .open(bash_file.clone()).ok()?;

    let mut content = String::new();
    file.read_to_string(&mut content).ok()?;
    let configured = content.contains(env_path_str.as_str());

    match mode {
        ConfigMode::INSTALL => {
            if !configured {
                file.write_fmt(format_args!("{}", env_path_str)).ok()?;
                println!("[INFO] Folder {} added to PATH", binary_folder_str);
            }
        },
        ConfigMode::UNINSTALL => {
            if configured {
                let final_string = content.replace(env_path_str.as_str(), "");
                let mut file = std::fs::OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(bash_file).ok()?;
                file.write_all(final_string.as_bytes()).ok()?;
                println!("[INFO] Folder {} removed to PATH", binary_folder_str)
            }
        },
    }

    Some(())
}