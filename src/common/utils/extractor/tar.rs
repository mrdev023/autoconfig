use std::{fs::File, path::{Path, PathBuf}};
use flate2::read::GzDecoder;
use tar::Archive;
use crate::common;


pub fn extract_file(path: &PathBuf, outdir: &str) -> Option<()> {
    let file = File::open(path).ok()?;
    let tar = GzDecoder::new(file);
    let mut archive = Archive::new(tar);
    let path = Path::new(common::TEMP_FOLDER).join(outdir);
    archive.unpack(path).ok()?;

    Some(())
}