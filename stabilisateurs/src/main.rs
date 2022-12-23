/// * `n` - nombre d'accroches
/// * `k` - nombre de stabilisateurs
/// * `p` - indice de stabilité parfaite
/// * `accroches` - hauteur de chaque accroche

fn stabilite_maximale(n: usize, k: usize, p: i64, mut accroches: Vec<i64>) {
    if n >= 4 && k > 0 {
        accroches.sort_unstable(); //sorting the values to calculate less groups (most stables groups are consecutive values)
        let mut all_costs = Vec::with_capacity(n - 3);
        for i in 0..(n - 3) {
            let cost = p - (accroches[i + 3] - accroches[i]).pow(2);
            if cost > 0 {
                all_costs.push(cost);
            }
        }
        println!("{}", find_max_stab(&all_costs[..], k));
    } else {
        println!("{}", 0);
    }
}

fn find_max_stab(stabs : &[i64], k: usize) -> i64 {
    let mut res : i64 = 0;
    let stabs_len = stabs.len();
    for i in 0..stabs_len {
        let mut stab = stabs[i];
        if i + 4 < stabs_len && k > 1 {
            stab += find_max_stab(&stabs[(i+4)..], k - 1);
        }
        if stab > res {
            res = stab;
        }
    }
    res
}

fn main() {
    let mut buffer = String::new();

    let n = read_line(&mut buffer)
        .parse()
        .expect("invalid `N` parameter");

    let k = read_line(&mut buffer)
        .parse()
        .expect("invalid `K` parameter");

    let p = read_line(&mut buffer)
        .parse()
        .expect("invalid `P` parameter");

    let accroches = read_line(&mut buffer)
        .split_whitespace()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .expect("invalid `accroches` parameter");

    stabilite_maximale(n, k, p, accroches);
}

fn read_line(buffer: &mut String) -> &str {
    buffer.clear();
    std::io::stdin()
        .read_line(buffer)
        .expect("impossible to read a new line");
    buffer.trim_end()
}