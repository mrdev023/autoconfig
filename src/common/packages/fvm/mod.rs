mod git;

use crate::common::utils::downloader;
use crate::common::utils::extractor;
use crate::common::utils::installer;

pub fn install() -> Result<(), String> {
    println!("Installing FVM");

    let url = git::get_file_url()?;

    let filename = url.split('/').last()
                        .ok_or(format!("Failed to download file"))?;

    let file = downloader::download_file(&url, &filename)
                    .ok_or(format!("Failed to download file"))?;

    extractor::extract_file(&file, "fvm")?;

    installer::install("fvm/fvm", installer::InstallType::Command).ok_or(format!("Failed to install"))?;

    println!("FVM Installed");

    Ok(())
}