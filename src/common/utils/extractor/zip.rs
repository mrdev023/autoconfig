use crate::common;
use std::{fs::{
        File,
        create_dir_all
    }, io, path::{self, PathBuf}};

pub fn extract_file(path: &PathBuf, outdir: &str) -> Option<()> {
    let file = File::open(&path).ok()?;
    let mut archive = match zip::ZipArchive::new(file) {
        Ok(archive) => archive,
        Err(err) => {
            println!("[ERROR][EXTRACTOR] {}", err);
            return None
        }
    };

    let path = path::Path::new(common::TEMP_FOLDER).join(outdir);

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).ok()?;
        let file_path = file.enclosed_name()?;
        let output_path = path.join(file_path);

        if (&*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, output_path.display());
            create_dir_all(&output_path).ok()?;
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                output_path.display(),
                file.size()
            );
            if let Some(p) = output_path.parent() {
                if !p.exists() {
                    create_dir_all(&p).ok()?;
                }
            }
            let mut outfile = File::create(&output_path).ok()?;
            io::copy(&mut file, &mut outfile).ok()?;
        }
    }

    Some(())
}