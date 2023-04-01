use args::Commands;
use clap::Parser;

mod args;
mod build;

fn main() {
    let args = args::Args::parse();

    match args.cmd {
        Commands::Build { path } => build::run(path),
        _ => todo!()
    }
}
