use std::path::{PathBuf, Path};
use std::{io, fs};

pub enum InstallType {
    Command,
    Config,
    Root
}

pub fn get_install_dir(install_type: InstallType) -> Result<PathBuf, String> {
    let home_dir = dirs::home_dir()
                            .ok_or(format!("Failed to get home_directory"))?;

    let subfolder = match install_type {
        InstallType::Command => "bin",
        InstallType::Config => "configs",
        InstallType::Root => ""
    };

    Ok(home_dir.join(super::INSTALL_FOLDER).join(subfolder))
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

pub fn install(source_path: &str, install_type: InstallType) -> Option <()> {
    let source_folder = std::path::Path::new(super::TEMP_FOLDER).join(source_path);

    let install_folder = get_install_dir(install_type).ok()?;
    copy_dir_all(source_folder, install_folder).ok()?;
    Some(())
}