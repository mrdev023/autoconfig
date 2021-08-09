use clap::{Clap};

use crate::installer::{
    Installer,
    utils::{
        configure::{
            configure,
            ConfigMode
        }
    }
};

mod installer;
mod packages;


#[derive(Clap, Debug)]
#[clap(name = "autoconfig")]
struct CommandArgs {
    #[clap(short, long)]
    install: Option<String>,
}

fn process() -> Result<(), String> {
    let command_args = CommandArgs::parse();

    match command_args.install {
        Some(package) => match package.as_str() {
            "fvm" => installer::process(Installer::GIT(packages::fvm::get_fvm_config()), ConfigMode::INSTALL),
            package => Err(format!("Package {} not found", package))
        },
        None => {
            println!("[INFO] Package list : [fvm]");
            Ok(())
        }
    }
}

fn main() {
    if let Err(err) = configure(&ConfigMode::INSTALL) {
        eprintln!("[ERROR] {}", err);
        return;
    }

    if let Err(err) = process() {
        eprintln!("[ERROR] {}", err);
        return;
    }
}
