use std::path::PathBuf;

use anyhow::{anyhow, Result};
use structopt::StructOpt;

fn main() -> Result<()> {
    let opt = Opt::from_args();
    init_log(opt.verbose)?;

    println!("Hello, world!");
    Ok(())
}

#[derive(Debug, StructOpt)]
#[structopt(about = "An example of StructOpt usage.")]
struct Opt {
    #[structopt(short = "v", parse(from_occurrences))]
    verbose: u8,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    Link {
        src: PathBuf,
        dst: PathBuf,
        exclusive: Vec<PathBuf>,
        inclusive: Vec<PathBuf>,
    },
}

fn init_log(verbose: u8) -> Result<()> {
    if verbose > 4 {
        return Err(anyhow!("invalid arg: 4 < {} number of verbose", verbose));
    }
    let level: log::LevelFilter = unsafe { std::mem::transmute((verbose + 1) as usize) };
    env_logger::builder()
        .filter_level(log::LevelFilter::Error)
        .filter_module(module_path!(), level)
        .init();
    Ok(())
}
