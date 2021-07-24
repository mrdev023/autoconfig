mod git;

use crate::common::utils::downloader;
use crate::common::utils::extractor;

// use std::process::Command;

pub fn install() -> Result<(), String> {
    println!("Install FVM");

    let url = git::get_file_url()?;

    let filename = url.split('/').last()
                        .ok_or(format!("Failed to download file"))?;

    let file = downloader::download_file(&url, &filename)
                    .ok_or(format!("Failed to download file"))?;

    extractor::extract_file(&file, "fvm")
                .ok_or(format!("Failed to extract file"))?;

    println!("{}", url);

    // let output = Command::new("winget")
    //                     .args(&[""])
    //                     .output()
    //                     .map_err(|_| format!("Permision refused ! Please run as administrator."))?;

    
    // println!("{}", std::str::from_utf8(&output.stdout).map_err(|_| format!("Internal error"))?);

    Ok(())
}