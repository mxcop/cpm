use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "cpm")]
#[command(author = "Max (mxcop) <mxcop.dev@gmail.com>")]
#[command(version = "0.0.0")]
#[command(about = "A basic C/C++ package manager", long_about = None)]
pub struct Args {
  /// Command to execute
  #[command(subcommand)]
  pub cmd: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a new cpm project.
    Create {
        name: String,
    },
    /// Build the cpm project.
    Build {
        path: Option<String>
    },
    /// Run the cpm project.
    Run
}