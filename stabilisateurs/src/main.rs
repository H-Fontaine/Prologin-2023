/// * `n` - nombre d'accroches
/// * `k` - nombre de stabilisateurs
/// * `p` - indice de stabilité parfaite
/// * `accroches` - hauteur de chaque accroche

fn stabilite_maximale(n: usize, k: usize, p: i32, mut accroches: Vec<i32>) {
    accroches.sort_unstable(); //sorting the values to calculate less groups (most stables groups are consecutive values)
    let all_groups = find_all_groups(n, k, &accroches[..]);
    let mut res = 0;
    for group in all_groups {
        let mut group_value = 0;
        for distance in group {
            let stability = p - distance * distance;
            if stability > 0 {
                group_value += stability;
            }
        }
        if group_value >= res {
            res = group_value;
        }
    }
    println!("{}", res);
}

fn find_all_groups(n: usize, k: usize, accroches : &[i32]) -> Vec<Vec<i32>> {
    if n <= 3 || k == 0 {
        vec![vec![]]
    } else {
        let mut res = find_all_groups(n - 4, k - 1, &accroches[4..]);
        if res[0].len() != 0 { //If there is sub groups
            res.push(vec![]);
        }
        for group in &mut res {
            group.push(accroches[3] - accroches[0])
        }
        for i in 1..(n-3) {
            let mut sub_groups = find_all_groups(n - 4 - i, k - 1, &accroches[(4 + i)..]);
            if sub_groups[0].len() != 0 {
                sub_groups.push(vec![]);
            }
            res.extend(sub_groups.into_iter().map(|mut a| {
                a.push(accroches[i + 3] - accroches[i]);
                a}));
        }
        res
    }
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