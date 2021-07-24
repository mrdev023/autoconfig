#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "macos")]
mod macos;

pub mod common;

fn main() {
    if let Err(err) = common::utils::configure::configure(&common::utils::configure::ConfigMode::INSTALL) {
        eprintln!("[ERROR] {}", err);
        return;
    }

    #[cfg(target_os = "windows")]
    windows::start();

    #[cfg(target_os = "linux")]
    linux::start();

    #[cfg(target_os = "macos")]
    macos::start();

    #[cfg(not(target_os = "windows"))]
    #[cfg(not(target_os = "linux"))]
    #[cfg(not(target_os = "macos"))]
    println!("[ERROR] Operating system not supported");
}
