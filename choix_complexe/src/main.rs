// 1- Store entry in 2 hashsets
// 2- Calculate the intersection of the two hashsets
// 3- The number of movies minus the size of the intersected hashsets


use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let mut buffer = String::new();

    // 1- Store entry in 2 hashsets
    let set_adore: HashSet<String> = HashSet::from_iter((0..6).map(|_| read_line(&mut buffer).to_string()));
    let set_deteste : HashSet<String> = HashSet::from_iter((0..6).map(|_| read_line(&mut buffer).to_string()));

    // 2- Calculate the intersection of the two hashmaps
    let intersection = set_adore.intersection(&set_deteste);

    // 3- The number of movies minus the size of the intersected hashsets
    println!("{}", 6 - intersection.count());
}

fn read_line(buffer: &mut String) -> &str {
    buffer.clear();
    std::io::stdin()
        .read_line(buffer)
        .expect("impossible to read a new line");
    buffer.trim_end()
}