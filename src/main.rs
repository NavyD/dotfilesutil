use std::path::PathBuf;

use anyhow::{anyhow, Result};
use dotfileutil::batchlink;
use structopt::StructOpt;

fn main() -> Result<()> {
    let opt = Opt::from_args();
    init_log(opt.verbose)?;
    match opt.cmd {
        Command::Link {
            dst,
            src,
            exclusive,
            inclusive,
            recursively,
            force,
        } => match batchlink::links(src, dst, &inclusive, &exclusive, recursively, force) {
            Ok(links) => {
                for (from, to) in links {
                    println!(
                        "linking {} to {}",
                        from.to_str().unwrap(),
                        to.to_str().unwrap()
                    );
                }
            }
            Err(e) => {
                eprintln!("failed to link: {}", e);
                return Err(e);
            }
        },
    }
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
        #[structopt(short)]
        src: PathBuf,
        #[structopt(short)]
        dst: PathBuf,
        #[structopt(short)]
        exclusive: Vec<PathBuf>,
        #[structopt(short)]
        inclusive: Vec<PathBuf>,
        #[structopt(short)]
        recursively: bool,
        #[structopt(short)]
        force: bool,
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
