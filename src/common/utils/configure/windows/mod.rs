mod env;

pub fn configure() -> Option<()> {
    env::configure_env(env::ConfigEnvMode::ADD)?;

    Some(())
}