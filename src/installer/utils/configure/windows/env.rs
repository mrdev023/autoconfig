use winreg::RegKey;
use winreg::enums::*;
use super::ConfigMode;
use super::super::super::file_utils;

fn env_exist(path_env: &str, binary_folder: &str) -> bool {
    for env in path_env.split(';') {
        if env == binary_folder {
            return true
        }
    }

    false
}

pub fn configure_env(mode: &ConfigMode) -> Option<()> {
    let hklm = RegKey::predef(HKEY_CURRENT_USER);
    let environment = hklm.open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE).ok()?;
    let mut reg_value : String = environment.get_value("PATH").ok()?;
    let binary_folder_path = file_utils::get_install_dir(file_utils::InstallType::Command).ok()?;
    let binary_folder_str = binary_folder_path.to_str()?;

    match mode {
        ConfigMode::INSTALL => {
            if !env_exist(reg_value.as_str(), binary_folder_str) {
                reg_value.push_str(format!(";{}", binary_folder_str).as_str()); // Add binary folder to path
                environment.set_value("PATH", &reg_value).ok()?;
                println!("[INFO] Folder {} added to PATH", binary_folder_str);
            }
        },
        ConfigMode::UNINSTALL => {
            if env_exist(reg_value.as_str(), binary_folder_str) {
                let new_value = reg_value.replace(format!(";{}", binary_folder_str).as_str(), ""); // Remove binary folder to path
                environment.set_value("PATH", &new_value).ok()?;
                println!("[INFO] Folder {} removed to PATH", binary_folder_str);
            }
        }
    }

    Some(())
}