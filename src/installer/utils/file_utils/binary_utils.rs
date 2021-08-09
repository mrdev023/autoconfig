use std::env::consts;
use std::path::PathBuf;
use std::fs;

#[cfg(target_os = "windows")]
use std::os::windows::fs::symlink_file;

fn file_is_executable(path: &PathBuf) -> bool {
    if consts::FAMILY == "windows" {
        match path.extension() {
            Some(ext) => {
                let extension = match ext.to_str() {
                    Some(r) => r,
                    None => ""
                };
                match extension {
                    "bat" | "exe" | "ps1" => true,
                    _ => false,
                }
            },
            None => false
        }
    } else {
        todo!()
    }
}

fn get_binary_files(install_package_folder: &PathBuf) -> Result<Vec<PathBuf>, String> {
    let mut files: Vec<PathBuf> = vec![];
    let paths = fs::read_dir(install_package_folder).map_err(|_| format!("Failed to get binary files"))?;

    for path_result in paths {
        let path = path_result.map_err(|_| format!("Failed to get file"))?;
        let metadata = path.metadata().map_err(|_| format!("Failed to get metadata"))?;
        if metadata.is_file() && file_is_executable(&path.path()) {
            files.push(path.path().clone());
        }
    }

    Ok(files)
}

pub fn create_symlink_of_binary_files(install_package_folder: &PathBuf, binary_package_folder: &PathBuf) -> Result<(), String> {
    let files = get_binary_files(install_package_folder)?;

    fs::create_dir_all(&binary_package_folder).map_err(|_| format!("Failed to create bin folder"))?;

    for file in files {
        let output_path = binary_package_folder.join(file.file_name().ok_or(format!("Failed to get filename"))?);

        #[cfg(target_os = "windows")]
        symlink_file(file.as_os_str(), output_path.as_os_str()).map_err(|_| format!("Failed to create symlink"))?;
    }

    Ok(())
}