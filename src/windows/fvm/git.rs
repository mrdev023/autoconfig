use crate::common::utils::git;

fn get_prefix_from_arch() -> Option<String> {
    if std::env::consts::ARCH == "x86_64" {
        Some(format!("x64"))
    } else if std::env::consts::ARCH == "x86" {
        Some(format!("ia32"))
    } else {
        None
    }
}

pub fn get_file_url() -> Result<String, String> {
    let git_response = git::get_git_latest_release("leoafarias/fvm")
                                        .ok_or(format!("Failed to get git release"))?;

    let arch_prefix = get_prefix_from_arch().ok_or(format!("Arch not supported"))?;

    for asset in git_response.assets {
        if asset.name.contains(std::env::consts::OS) && asset.name.contains(arch_prefix.as_str()) {
            return Ok(asset.browser_download_url);
        }
    }

    Err(format!("OS not supported"))
}