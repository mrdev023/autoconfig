use std::{fs, path::PathBuf};

fn create_download_folder() -> Option<()> {
    fs::create_dir_all(super::super::TEMP_FOLDER).ok()
}

pub fn download_file(url : &String, filename: &str) -> Option<PathBuf> {
    create_download_folder()?;
    let mut response = reqwest::blocking::get(url).ok()?;

    let fname = std::path::Path::new(super::super::TEMP_FOLDER).join(filename);

    let mut dest = fs::File::create(fname.clone()).ok()?;
    response.copy_to(&mut dest).ok()?;

    Some(fname)
}