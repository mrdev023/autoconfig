#[cfg(target_os = "windows")]
mod windows;

pub fn configure() -> Result<(), String> {
    #[cfg(target_os = "windows")]
    windows::configure().ok_or(format!("Failed to configure environment"))?;

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