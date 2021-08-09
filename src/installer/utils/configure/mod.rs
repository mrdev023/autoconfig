#[cfg(target_os = "windows")]
mod windows;

mod common;

pub enum ConfigMode {
    INSTALL,
    UNINSTALL
}

pub fn configure(mode: &ConfigMode) -> Result<(), String> {
    common::configure_folder(&mode).ok_or(format!("Failed to configure folder"))?;

    #[cfg(target_os = "windows")]
    windows::configure(&mode).ok_or(format!("Failed to configure environment"))?;

    #[cfg(not(target_os = "windows"))]
    #[cfg(not(target_os = "linux"))]
    #[cfg(not(target_os = "macos"))]
    {
        Err(format!("OS not supported"))
    }

    #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
    {
        Ok(())
    }
}