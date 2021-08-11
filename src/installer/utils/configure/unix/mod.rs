mod env;
use super::ConfigMode;

pub fn configure(mode: &ConfigMode) -> Option<()> {
    env::configure_env(mode)?;

    Some(())
}