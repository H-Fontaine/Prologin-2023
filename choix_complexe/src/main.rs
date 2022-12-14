// 1- Read the entry
// 2- Store entry in 2 hashsets
// 3- Calculate the intersection of the two hashsets
// 4- The number of movies minus the size of the intersected hashsets


use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, Cursor, Read};

fn main() {
    // 1- Read the entry
    let mut file= String::new();
    File::open("entries/entry1.txt").unwrap().read_to_string(&mut file).unwrap();
    let mut lines = Cursor::new(file).lines();

    // 2- store entry in 2 hashmaps
    let set_adore: HashSet<String> = HashSet::from_iter(lines.by_ref().map(|a| a.unwrap()).take(6));
    let set_deteste : HashSet<String> = HashSet::from_iter(lines.map(|a| a.unwrap()).take(6));

    // 3- Calculate the intersection of the two hashmaps
    let intersection = set_adore.intersection(&set_deteste);

    // 4- The number of lines in 1 hashmaps minus the size of the intersected hashmaps
    println!("{}", 6 - intersection.count());
}
