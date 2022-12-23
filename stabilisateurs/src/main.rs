/// * `n` - nombre d'accroches
/// * `k` - nombre de stabilisateurs
/// * `p` - indice de stabilité parfaite
/// * `accroches` - hauteur de chaque accroche

fn stabilite_maximale(n: usize, k: usize, p: i32, mut accroches: Vec<i32>) {
    accroches.sort_unstable(); //sorting the values to calculate less groups (most stables groups are consecutive values)
    let mut all_distances = Vec::with_capacity(n - 3);
    for i in 0..(n - 3) {
        all_distances.push(accroches[i + 3] - accroches[i]);
    }
    let all_groups = find_all_groups(all_distances.len(), k, all_distances);
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

fn find_all_groups(n : usize, mut k: usize, input : Vec<i32>) -> Vec<Vec<i32>> {
    let mut res : Vec<Vec<usize>> = (0..n).map(|i| vec![i]).collect();
    let mut next= Vec::new();
    if n >= 4 {
        res[..(n-4)].clone_into(&mut next);
    }
    k -=1;

    while next.len() > 0 && k > 0 {
        let next_old = next;
        next = Vec::new();
        for group in next_old {
            let last_number = group.last().unwrap();
            for number in (last_number + 4)..n {
                let mut group_cloned = group.clone();
                group_cloned.push(number);
                res.push(group_cloned);
                if number + 4 < n {
                    next.push(res.last().unwrap().clone());
                }
            }
        }
        k -= 1;
    }
    res.into_iter().map(|group| group.into_iter().map(|id| input[id]).collect()).collect()
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