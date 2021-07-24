
use structopt::{
    StructOpt,
    clap::{
        arg_enum
    }
};

arg_enum! {
    #[derive(Debug)]
    enum Tool {
        Fvm
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "autoconfig", about = "Auto install local config.")]
struct Opt {
    #[structopt(short = "i", long = "install", help = "Available values : fvm (Flutter version manager)")]
    tools: Vec<Tool>,
}


pub fn start() {
    let opt = Opt::from_args();
    for tool in &opt.tools {
        let result = match tool {
            Tool::Fvm => super::common::packages::fvm::install()
        };

        if let Err(err) = result {
            eprintln!("[ERROR] {}", err);
        }
    }
}