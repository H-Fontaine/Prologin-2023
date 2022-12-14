/// * `n` - Le nombre de boîtes et de restes
/// * `restes` - Liste des volumes des restes
/// * `boites` - Liste des volumes des boîtes

fn main() {
    let mut buffer = String::new();

    let n = read_line(&mut buffer)
        .parse()
        .expect("invalid `N` parameter");

    let mut restes : Vec<u32> = read_line(&mut buffer)
        .split_whitespace()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .expect("invalid `restes` parameter");

    let mut boites : Vec<u32> = read_line(&mut buffer)
        .split_whitespace()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .expect("invalid `boites` parameter");

    //Sorting of restes and boites (sort_unstable a little faster than sort)
    restes.sort_unstable();
    boites.sort_unstable();

    //Because restes and boites are sorted we are sure to store as many restes as possible because restes are always put in the smallest box possible for them (excepting the ones already taken)

    let mut restes_indice = 0;
    let mut current_reste_size = restes[0];
    let mut boites_indice = 0;
    let mut res = 0;
    while restes_indice < n && boites_indice < n {
        if restes[restes_indice] <= boites[boites_indice] {
            res += 1;
            restes_indice += 1;
            boites_indice += 1;
            current_reste_size = restes[restes_indice];
        } else {
            while boites_indice < n && boites[boites_indice] < current_reste_size {
                boites_indice += 1;
            }
        }
    }
    println!("{}", res)
}

fn read_line(buffer: &mut String) -> &str {
    buffer.clear();
    std::io::stdin()
        .read_line(buffer)
        .expect("impossible to read a new line");
    buffer.trim_end()
}
