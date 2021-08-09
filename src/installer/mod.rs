pub mod git;
pub mod utils;

use utils::configure::ConfigMode;

use self::git::GitConfig;


pub enum Installer {
    GIT(GitConfig),
}

pub fn process(installer: Installer, config_mode: ConfigMode) -> Result<(), String> {
    match installer {
        Installer::GIT(config) => git::process(config, config_mode),
    }
}