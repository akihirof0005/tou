use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

//座標
#[derive(Debug)]
struct Point3d {
    x: f64,
    y: f64,
    z: f64,
}
//Atom
#[derive(Debug)]
struct Atom {
    index: u32,
    cordinate: Point3d,
    symbol: String,
    links: Vec<String>,
}
//外積
fn veprod(v1: &Point3d, v2: &Point3d) -> Point3d {
    let x: f64 = v1.y * v2.z - v2.y * v1.z;
    let y: f64 = v1.z * v2.x - v2.z * v1.x;
    let z: f64 = v1.x * v2.y - v2.x * v1.y;
    return Point3d { x: x, y: y, z: z };
}
//内積
fn scprod(v1: &Point3d, v2: &Point3d) -> f64 {
    let ret: f64 = v1.x * v2.x + v1.y * v2.y + v1.z * v2.z;
    return ret;
}
//差（座標をベクトルにする）
fn sub(p1: &Point3d, p2: &Point3d) -> Point3d {
    let x: f64 = p2.x - p1.x;
    let y: f64 = p2.y - p1.y;
    let z: f64 = p2.z - p1.z;
    return Point3d { x: x, y: y, z: z };
}
//文字列から文字数分取り出す
fn kiridashi(text: String, start: usize, end: usize) -> String {
    let begin = text.char_indices().nth(start).unwrap().0;
    let end = text.char_indices().nth(end).unwrap().0;
    let ret = &text[begin..end];
    return ret.trim().to_owned();
}
//
fn tou_reader(path: String) -> Vec<String> {
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
            let index: usize = links.get(0) as usize;
            atoms[index].links = links;
            println!("{:?}", items);
            continue;
        }
        if "HET" == rec {
            //println!("{:?}", items);
            let num: u32 = items[1].parse().unwrap();
            let cord: Point3d = Point3d {
                x: items[5].parse().unwrap(),
                y: items[6].parse().unwrap(),
                z: items[7].parse().unwrap(),
            };
            atoms.push(Atom {
                index: num,
                symbol: items[2].to_string(),
                cordinate: cord,
                links: Vec::new(),
            })
        } else {
            println!("{}", s);
            //continue;
        }
    }
    //println!("{:?}", atoms);
    return list;
}

fn main() {
    let path = format!("{}", "/home/skit/src/akihirof0005/tou/a-D-Manp.pdb");
    let list = tou_reader(path);
}
