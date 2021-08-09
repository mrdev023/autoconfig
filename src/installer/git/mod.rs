use std::env::consts;

use super::utils::{configure, downloader, extractor, git, file_utils};

pub struct GitFileIdentifier {
    pub os_name: String,
    pub arch: String,
    pub os_identifier: String,
    pub arch_identifier: String,
}

/// Git configuration structure
pub struct GitConfig {
    /// base url from git url (ex: fvm url is https://github.com/leoafarias/fvm so the package name is leoafarias/fvm)
    pub package: String,
    /// Must exist in Release of git repo (default to latest)
    pub version: String,
    pub git_identifiers: Vec<GitFileIdentifier>,
    pub archive_subfolder: String,
    pub install_type: file_utils::InstallType,
}

fn get_file_url(package: &str, version: &str, git_identifiers: Vec<GitFileIdentifier>) -> Result<String, String> {
    let git_response = git::get_git_release_by_version(package, version)
                                        .ok_or(format!("Failed to get git release"))?;

    let git_identifier  = git_identifiers.into_iter()
                                                .find(|id| {
                                                    id.arch == consts::ARCH && (id.os_name == consts::OS || id.os_name == consts::FAMILY)
                                                })
                                                .ok_or(format!("CPU ARCH or OS not supported"))?;

    for asset in git_response.assets {
        if asset.name.contains(git_identifier.arch_identifier.as_str()) && asset.name.contains(git_identifier.os_identifier.as_str()) {
            return Ok(asset.browser_download_url);
        }
    }

    Err(format!("{} not support current OS or CPU ARCH", package))
}

fn install(config: GitConfig) -> Result<(), String> {
    let url = get_file_url(config.package.as_str(), config.version.as_str(), config.git_identifiers)?;

    let filename = url.split('/').last()
                        .ok_or(format!("Failed to get name of file"))?;

    let file = downloader::download_file(&url, &filename)
                        .ok_or(format!("Failed to download file"))?;

    extractor::extract_file(&file, config.package.as_str())?;

    let package_temp_folder = std::path::Path::new(config.package.as_str());
    let extractor_package_temp_folder = package_temp_folder.join(config.archive_subfolder.as_str());
    let extractor_package_temp_folder_str = extractor_package_temp_folder
                                .as_os_str()
                                .to_str()
                                .ok_or(format!("Failed to get extracted folder"))?;

    file_utils::install(extractor_package_temp_folder_str, config.package.as_str(), config.install_type).ok_or(format!("Failed to install"))?;

    Ok(())
}

pub fn process(config: GitConfig, config_mode: configure::ConfigMode) -> Result<(), String> {
    match config_mode {
        configure::ConfigMode::INSTALL => install(config),
        configure::ConfigMode::UNINSTALL => Err(format!("not yet implemented")),
    }
}