use std::path::PathBuf;
use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Cli {
    name: Option<String>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    pub(crate) command: Option<Commands>,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    Archive {
        #[arg(short, long)]
        files: Vec<PathBuf>,
        output: PathBuf,
        compression: Option<String>,
        encrypt: Option<bool>,
        password: Option<String>
    },
    Decompress{
        input: PathBuf,
        compression: Option<String>,
        decrypt: Option<bool>,
        password: Option<String>
    }
}
