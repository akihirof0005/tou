use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
///座標を代表する
struct Point3d {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug)]
///分子中の原子を記述する、シンボル、添字、座標、結合先のAtomの添字の配列を格納する
struct Atom {
    index: u32,
    coordinate: Point3d,
    symbol: String,
    links: Vec<i32>,
}

///座標を位置ベクトルと捉えた場合の外積を計算するメソッド
fn veprod(v1: &Point3d, v2: &Point3d) -> Point3d {
    let x: f64 = v1.y * v2.z - v2.y * v1.z;
    let y: f64 = v1.z * v2.x - v2.z * v1.x;
    let z: f64 = v1.x * v2.y - v2.x * v1.y;
    return Point3d { x: x, y: y, z: z };
}

///座標を位置ベクトルと捉えた場合の内積を計算するメソッド
fn scprod(v1: &Point3d, v2: &Point3d) -> f64 {
    let ret: f64 = v1.x * v2.x + v1.y * v2.y + v1.z * v2.z;
    return ret;
}

///座標を位置ベクトルと捉えた場合の差を計算するメソッド
fn sub(p1: &Point3d, p2: &Point3d) -> Point3d {
    let x: f64 = p2.x - p1.x;
    let y: f64 = p2.y - p1.y;
    let z: f64 = p2.z - p1.z;
    return Point3d { x: x, y: y, z: z };
}

///Stringから文字数分取り出す
fn kiridashi(text: String, start: usize, end: usize) -> String {
    let begin = text.char_indices().nth(start).unwrap().0;
    let end = text.char_indices().nth(end).unwrap().0;
    let ret = &text[begin..end];
    return ret.trim().to_owned();
}

///PDBファイルからVec<Atom>に読み込む
fn tou_reader(path: String) -> Vec<Atom> {
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

///Vec<Atom>から-NH2(アミン基)をカウントする
fn is_included_nh2(tou: &Vec<Atom>) -> i32 {
    let mut ret: i32 = 0;
    for atom in tou {
        if atom.symbol == "N" && atom.links.len() == 3 {
            if (tou[(atom.links[0] - 1) as usize].symbol == "C"
                && tou[(atom.links[1] - 1) as usize].symbol == "H"
                && tou[(atom.links[2] - 1) as usize].symbol == "H")
                || (tou[(atom.links[0] - 1) as usize].symbol == "H"
                    && tou[(atom.links[1] - 1) as usize].symbol == "C"
                    && tou[(atom.links[2] - 1) as usize].symbol == "H")
                || (tou[(atom.links[0] - 1) as usize].symbol == "H"
                    && tou[(atom.links[1] - 1) as usize].symbol == "H"
                    && tou[(atom.links[2] - 1) as usize].symbol == "C")
            {
                ret += 1;
            }
        }
    }
    return ret;
}

///Vec<Atom>から-OH(水酸基)をカウントする
fn is_included_oh(tou: &Vec<Atom>) -> i32 {
    let mut ret: i32 = 0;
    for atom in tou {
        if atom.symbol == "O" && atom.links.len() == 2 {
            if (tou[(atom.links[0] - 1) as usize].symbol == "C"
                && tou[(atom.links[1] - 1) as usize].symbol == "H")
                || (tou[(atom.links[0] - 1) as usize].symbol == "H"
                    && tou[(atom.links[1] - 1) as usize].symbol == "C")
            {
                ret += 1;
            }
        }
    }
    return ret;
}

///Vec<Atom>から-COOH(カルボン酸)をカウントする
fn is_included_cooh(tou: &Vec<Atom>) -> i32 {
    let mut ret: i32 = 0;
    for atom in tou {
        if atom.symbol == "O"
            && atom.links.len() == 1
            && tou[(atom.links[0] - 1) as usize].symbol == "C"
        {
            let carbon: &Atom = &tou[(atom.links[0] - 1) as usize];
            if carbon.links.len() == 3 {
                if (tou[(carbon.links[0] - 1) as usize].symbol == "C"
                    && tou[(carbon.links[1] - 1) as usize].symbol == "O"
                    && tou[(carbon.links[2] - 1) as usize].symbol == "O")
                    || (tou[(carbon.links[0] - 1) as usize].symbol == "O"
                        && tou[(carbon.links[1] - 1) as usize].symbol == "C"
                        && tou[(carbon.links[2] - 1) as usize].symbol == "O")
                    || (tou[(carbon.links[0] - 1) as usize].symbol == "O"
                        && tou[(carbon.links[1] - 1) as usize].symbol == "O"
                        && tou[(carbon.links[2] - 1) as usize].symbol == "C")
                {
                    ret += 1;
                }
            }
        }
    }
    return ret;
}

///Vec<Atom>から-NOCH3(Nアセチル)をカウントする
fn is_included_noc(tou: &Vec<Atom>) -> i32 {
    let mut ret: i32 = 0;
    for atom in tou {
        if atom.symbol == "O"
            && atom.links.len() == 1
            && tou[(atom.links[0] - 1) as usize].symbol == "C"
        {
            let carbon: &Atom = &tou[(atom.links[0] - 1) as usize];
            if carbon.links.len() == 3 {
                if (tou[(carbon.links[0] - 1) as usize].symbol == "C"
                    && tou[(carbon.links[1] - 1) as usize].symbol == "N"
                    && tou[(carbon.links[2] - 1) as usize].symbol == "O")
                    || (tou[(carbon.links[0] - 1) as usize].symbol == "C"
                        && tou[(carbon.links[1] - 1) as usize].symbol == "O"
                        && tou[(carbon.links[2] - 1) as usize].symbol == "N")
                    || (tou[(carbon.links[0] - 1) as usize].symbol == "N"
                        && tou[(carbon.links[1] - 1) as usize].symbol == "C"
                        && tou[(carbon.links[2] - 1) as usize].symbol == "O")
                    || (tou[(carbon.links[0] - 1) as usize].symbol == "N"
                        && tou[(carbon.links[1] - 1) as usize].symbol == "O"
                        && tou[(carbon.links[2] - 1) as usize].symbol == "C")
                    || (tou[(carbon.links[0] - 1) as usize].symbol == "O"
                        && tou[(carbon.links[1] - 1) as usize].symbol == "N"
                        && tou[(carbon.links[2] - 1) as usize].symbol == "C")
                    || (tou[(carbon.links[0] - 1) as usize].symbol == "O"
                        && tou[(carbon.links[1] - 1) as usize].symbol == "C"
                        && tou[(carbon.links[2] - 1) as usize].symbol == "N")
                {
                    ret += 1;
                }
            }
        }
    }
    return ret;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let tou: Vec<Atom> = tou_reader(filename.to_string());

    let oh_count = is_included_oh(&tou);
    let cooh_count = is_included_cooh(&tou);
    let noc_count = is_included_noc(&tou);
    let nh2_count = is_included_nh2(&tou);
    println!(
        "OH:{:?} COOH:{:?} NOC:{:?} NH2:{:?}",
        oh_count, cooh_count, noc_count, nh2_count
    );
}
