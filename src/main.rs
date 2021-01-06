extern crate tou;

mod finding_partial_structures;
mod manipulate_vectors;
mod manipulation;

mod structs;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let tou: Vec<structs::Atom> = manipulation::tou_reader(filename.to_string());

    let oh_count = finding_partial_structures::is_included_oh(&tou);
    let cooh_count = finding_partial_structures::is_included_cooh(&tou);
    let noc_count = finding_partial_structures::is_included_noc(&tou);
    let nh2_count = finding_partial_structures::is_included_nh2(&tou);
    println!(
        "OH:{:?} COOH:{:?} NOC:{:?} NH2:{:?}",
        oh_count, cooh_count, noc_count, nh2_count
    );
}
