use std::env;
use std::env::Args;
pub(crate) fn parse_archive(args: Args) -> (Vec<String>, String, bool) {
    let args: Vec<String> = args.collect();
    let mut is_still_file : bool  = false;
    let mut file_length = 0;
    let mut file_start = 0;
    let mut output_file  = String::new();
    let mut valid = true;
    let usage_message="Usage: ./main archive --files <file1> <file2> ... --output <output_file>";
    valid = args.contains(&String::from("archive"));
    valid = args.contains(&String::from("--files"));
    valid = args.contains(&String::from("--output"));

    for mut i in 1..args.len() {
        if args[i] == "--files"{
            is_still_file = true;
            i+=1;
            file_start=i;
        }
        if args[i] == "--output"{
            is_still_file = false;
            file_length-=1;
            output_file = args[i+1].clone();
        }
        if is_still_file {
            file_length+=1;
        }
    }
    if output_file == "" {
        valid = false;
    }
    if file_length == 0 {
        valid = false;
    }

    return (args[file_start..file_start+file_length].to_vec(), output_file,valid);
}

pub(crate) fn parse_dearchive(args: Args) -> (String, bool) {
    let args: Vec<String> = args.collect();
    let mut file_start = 0;
    let mut valid = true;
    valid = args.contains(&String::from("dearchive"));
    valid = args.contains(&String::from("--file"));
    for mut i in 0..args.len() {
        if args[i] == "--file"{
            file_start = i+1
        }
    }
    let output_file = args[file_start].clone();
    return (output_file,valid);
}