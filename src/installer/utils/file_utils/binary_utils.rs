use std::path::PathBuf;
use std::fs::{self, Metadata};

#[cfg(target_family = "unix")]
use std::os::unix::fs::symlink;

#[cfg(target_family = "unix")]
use std::os::unix::fs::PermissionsExt;

fn file_is_executable(path: &PathBuf, metadata: &Metadata) -> bool {
    #[cfg(target_family = "windows")]
    {
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
    }
    #[cfg(target_family = "unix")]
    {
        if metadata.permissions().mode() & 0o111 != 0 {
            true
        } else {
            false
        }
    }
}

fn get_binary_files(install_package_folder: &PathBuf) -> Result<Vec<PathBuf>, String> {
    let mut files: Vec<PathBuf> = vec![];
    let paths = fs::read_dir(install_package_folder).map_err(|_| format!("Failed to get binary files"))?;

    for path_result in paths {
        let path = path_result.map_err(|_| format!("Failed to get file"))?;
        let metadata = path.metadata().map_err(|_| format!("Failed to get metadata"))?;
        if metadata.is_file() && file_is_executable(&path.path(), &metadata) {
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

        #[cfg(target_family = "windows")]
        {
            let file_path = file.as_os_str().to_str().ok_or(format!("Failed to get filename str"))?;
            fs::write(output_path, format!("
            @echo off
            {} %*
            ", file_path)).map_err(|_| format!("Failed to create symlink"))?;
        }

        #[cfg(target_family = "unix")]
        symlink(file.as_os_str(), output_path.as_os_str()).map_err(|_| format!("Failed to create symlink"))?;
    }

    Ok(())
}