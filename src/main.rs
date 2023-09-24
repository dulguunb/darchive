use std::env;
mod arg_parser;
mod archive;
use archive::{archive_files,de_archive_files};
use crate::arg_parser::{parse_archive, parse_dearchive};

fn main() {
    let (files, archive,valid) = parse_archive(env::args());
    if valid {
        println!("files: {:?}", files);
        println!("archive: {:?}", archive);
        archive_files(files, archive)
    }
    else {
        let (output_file,valid) = parse_dearchive(env::args());
        de_archive_files(output_file);
        if valid {
        }
        else {
            println!("Invalid arguments");
        }
    }

}
