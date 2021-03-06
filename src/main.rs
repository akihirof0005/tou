extern crate tou;

mod finding_partial_structures;
mod manipulate_vectors;
mod manipulation;

mod structs;

//use std::env;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

use rustc_serialize::json;

fn main() {
    let paths = fs::read_dir("./pdb").unwrap();
    let mut files: Vec<String> = vec![];
    for path in paths {
        files.push(path.unwrap().path().display().to_string());
    }

    //   let args: Vec<String> = env::args().collect();
    let mut result: HashMap<String, HashMap<&str, i32>> = HashMap::new();
    for file in files.iter() {
        let filename = &file;
        let tou: Vec<structs::Atom> = manipulation::tou_reader(filename.to_string());

        let oh_count: i32 = finding_partial_structures::is_included_oh(&tou);
        let cooh_count: i32 = finding_partial_structures::is_included_cooh(&tou);
        let noc_count: i32 = finding_partial_structures::is_included_noc(&tou);
        let nh2_count: i32 = finding_partial_structures::is_included_nh2(&tou);
        let mut tou: HashMap<&str, i32> = HashMap::new();
        tou.insert("OH", oh_count);
        tou.insert("COOH", cooh_count);
        tou.insert("NOC", noc_count);
        tou.insert("NH2", nh2_count);

        let name: &OsStr = Path::new(&file).file_stem().unwrap();
        let name: String = Path::new(name).display().to_string();
        result.insert(name, tou);
    }
    println!("{}", json::encode(&result).unwrap());
}
