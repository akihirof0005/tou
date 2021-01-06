use crate::structs::Atom;
use crate::structs::Point3d;

//use std::collections::HashMap;
//use std::env;
//use std::fs;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

///Stringから文字数分取り出す
pub fn kiridashi(text: String, start: usize, end: usize) -> String {
    let begin = text.char_indices().nth(start).unwrap().0;
    let end = text.char_indices().nth(end).unwrap().0;
    let ret = &text[begin..end];
    return ret.trim().to_owned();
}

///PDBファイルからVec<Atom>に読み込む
pub fn tou_reader(path: String) -> Vec<Atom> {
    let path = Path::new(&path);
    let display = path.display();

    let f = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    //開いファイルを使ってBufReaderで読み出す
    let reader = BufReader::new(f);
    let mut list: Vec<String> = Vec::new();
    let mut atoms: Vec<Atom> = Vec::new();

    //一行ずつ取り出しながらベクターに格納していく
    for line in reader.lines() {
        let s = line.unwrap();
        if s == "" || s == "END" {
            continue;
        }
        let mut items: Vec<&str> = s.split_whitespace().collect();
        let rec: String = kiridashi(s.to_string(), 0, 3);

        if "CON" == rec {
            //list.push(s);
            items.remove(0);

            let mut links = Vec::<i32>::new();
            for item in items {
                links.push(item.parse().unwrap());
            }
            let index: usize = (links[0] - 1) as usize;
            links.remove(0);
            atoms[index].links = links;
            //           println!("{:?}", items);
            continue;
        }
        if "HET" == rec {
            //println!("{:?}", items);
            let num: u32 = items[1].parse().unwrap();
            let coord: Point3d = Point3d {
                x: items[5].parse().unwrap(),
                y: items[6].parse().unwrap(),
                z: items[7].parse().unwrap(),
            };
            atoms.push(Atom {
                index: num,
                symbol: items[2].to_string(),
                coordinate: coord,
                links: Vec::new(),
            })
        } else {
            //println!("{}", s);
            //continue;
        }
    }
    //println!("{:?}", atoms);
    return atoms;
}
