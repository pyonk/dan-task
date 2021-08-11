mod cli;
mod item;
mod task;

use cli::{Cli, Command};
use structopt::StructOpt;

fn main() {
    let args = Cli::from_args();
    let task = task::from_storage(&args.path);
    match args.command {
        Some(cmd) => match cmd {
            Command::Add { args } => cli::add(task, args),
        },
        None => {
            cli::list(task);
        }
    }
}
