#[cfg(target_family = "windows")]
mod windows;

#[cfg(target_family = "unix")]
mod unix;

mod common;

pub enum ConfigMode {
    INSTALL,
    UNINSTALL
}

pub fn configure(mode: &ConfigMode) -> Result<(), String> {
    common::configure_folder(&mode).ok_or(format!("Failed to configure folder"))?;

    #[cfg(target_family = "windows")]
    windows::configure(&mode).ok_or(format!("Failed to configure environment"))?;

    #[cfg(target_family = "unix")]
    unix::configure(&mode).ok_or(format!("Failed to configure environment"))?;

    #[cfg(not(target_family = "windows"))]
    #[cfg(not(target_family = "unix"))]
    {
        Err(format!("OS not supported"))
    }

    #[cfg(any(target_family = "windows", target_family = "unix"))]
    {
        Ok(())
    }
}