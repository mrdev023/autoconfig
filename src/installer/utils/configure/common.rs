use std::path::Path;
use std::fs::{create_dir_all, remove_dir_all};

use super::ConfigMode;
use super::super::file_utils;

fn folder_exist(folder: &str) -> bool {
    Path::new(folder).is_dir()
}

pub fn configure_folder(mode: &ConfigMode) -> Option<()> {
    let binary_folder_path = file_utils::get_install_dir(&file_utils::InstallType::Root).ok()?;
    let binary_folder_str = binary_folder_path.to_str()?;

    match mode {
        ConfigMode::INSTALL => {
            if !folder_exist(binary_folder_str) {
                println!("[INFO] Create {} folder", binary_folder_str);
                create_dir_all(binary_folder_path).ok()?;
            }
        },
        ConfigMode::UNINSTALL => {
            if folder_exist(binary_folder_str) {
                println!("[INFO] Remove {} folder", binary_folder_str);
                remove_dir_all(binary_folder_path).ok()?;
            }
        }
    }

    Some(())
}