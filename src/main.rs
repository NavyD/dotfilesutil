use std::{path::PathBuf, process::exit};

use anyhow::{anyhow, Result};
use dotfileutil::{batchlink, completions::gen_zsh_cmd_completion};
use structopt::StructOpt;

fn main() {
    let opt = Opt::from_args();
    init_log(opt.verbose).expect("init log");
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
                exit(1)
            }
        },
        Command::Completions { gen } => match gen {
            CompletionsGen::Cmd {
                completion_option,
                name,
                version_option,
            } => match gen_zsh_cmd_completion(&name, &version_option, &completion_option) {
                Ok(s) => println!("{}", s),
                Err(e) => {
                    eprintln!("{}", e);
                    exit(1);
                }
            },
        },
    }
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
    Completions {
        #[structopt(subcommand)]
        gen: CompletionsGen,
    },
}

#[derive(Debug, StructOpt)]
enum CompletionsGen {
    /// Generate updatable Zsh complements for commands with complements。
    ///
    /// usage:
    ///
    /// <bin> completions cmd -n poetry -v='--version' -c="completions zsh"
    ///
    /// Note that `-v --version` cannot be parsed in structopt, use `=` to separate
    Cmd {
        #[structopt(short)]
        name: String,

        #[structopt(short)]
        version_option: String,

        #[structopt(short)]
        completion_option: String,
    },
}

fn init_log(verbose: u8) -> Result<()> {
    env!("PATH");
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
