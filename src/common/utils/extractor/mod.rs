use std::path::{PathBuf};

mod zip;
mod tar;

pub fn extract_file(path: &PathBuf, outdir: &str) -> Result<(), String> {
    let file_extension = path.extension()
                                    .ok_or(format!("Failed to get extension"))?
                                    .to_str()
                                    .ok_or(format!("Failed to convert extension to &str"))?;
    match file_extension {
        "zip" => Ok(zip::extract_file(&path, outdir).ok_or(format!("Extract failed"))?),
        "bz2" => Ok(zip::extract_file(&path, outdir).ok_or(format!("Extract failed"))?),
        "tar" => Ok(tar::extract_file(&path, outdir).ok_or(format!("Extract failed"))?),
        "gz" => Ok(tar::extract_file(&path, outdir).ok_or(format!("Extract failed"))?),
        _ => Err(format!("Format not supported"))
    }
}