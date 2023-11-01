use std::fs::{File};
use std::fs as fs1;
use std::io::prelude::*;
use std::io::Write;
use std::os::linux::fs::MetadataExt;
use Vec;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Archive {
    filenames: Vec<String>,
    contents: Vec<Vec<u8>>,
    g_ids: Vec<u32>,
    u_ids: Vec<u32>,
    c_times: Vec<i64>,
}
pub(crate) fn archive_files(filepaths: Vec<String>, output_file: String){
    let mut file = File::create(output_file.clone()).expect("Unable to create file");
    let mut contents = Vec::new();
    let mut archive = Archive{
        filenames: filepaths.clone(),
        contents: Vec::new(),
        g_ids: Vec::new(),
        u_ids: Vec::new(),
        c_times: Vec::new(),
    };

    for file in &filepaths {
        println!("file: {}", file);
        let content = fs1::read(file.clone()).expect("Unable to read file");
        let meta = fs1::metadata(file.clone()).expect("Unable to read metadata");
        meta.permissions().set_readonly(true);
        archive.contents.push(content);
    }
    let encoded_header: Vec<u8> = bincode::serialize(&archive).unwrap();
    contents.push(encoded_header);
    file.write_all(contents.concat().as_slice()).expect("Unable to write data" )
}

pub(crate) fn de_archive_files(file_path: String){
    let mut file = File::open(file_path.clone()).expect("Unable to open file");
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).expect("Unable to read file");
    let header: Archive = bincode::deserialize(&contents).unwrap();
    for i in 0..header.filenames.len() {
        let mut out_file = File::create(header.filenames[i].clone()).expect("Unable to create file");
        out_file.write(header.contents[i].as_slice()).expect("Unable to write data" );
        out_file.metadata().unwrap().st_mtime_nsec();
        println!("filename: {}", header.filenames[i]);
    }
}