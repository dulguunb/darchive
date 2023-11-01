use std::env;
use clap::Parser;

mod arg_parser;
mod archive;
mod huffman;

use archive::{archive_files,de_archive_files};
use arg_parser::Cli;
use crate::arg_parser::Commands;
use huffman::create_freq_for_ascii;
fn main() {
    // let cli = Cli::parse();
    // match &cli.command {
    //     Some(Commands::Archive { output, .. }) => {
    //         println!("{:?}", output.to_str())
    //     }
    //     Some(Commands::Archive {compression, .. }) => {
    //         println!("{:?}",compression)
    //     }
    //     Some(Commands::Archive {files, .. }) => {
    //         let files_str = ".";
    //         println!("{:?}",files_str);
    //     }
    //     None => {}
    //     _ => {}
    // }
    // create_freq_for_ascii("helloworld.txt")
}
