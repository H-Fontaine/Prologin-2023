use std::collections::HashSet;

/// * `n` - Le nombre de points
/// * `m` - Le nombre de tuyaux
/// * `k` - Le nombre de degrés minimum de refroidissement
/// * `a` - Le point de départ
/// * `b` - Le point d'arrivée
/// * `tuyaux` - Les tuyaux orientés (point de départ, point d'arrivée, refroidissement)

fn refroidissement(n: usize, _m: usize, k: usize, a: usize, b: usize, tuyaux: Vec<Vec<usize>>) {
    let mut followers_list = vec![Vec::<(usize, usize)>::new(); n + 1];
    for tuyau in tuyaux {
        followers_list[tuyau[0]].push((tuyau[1], tuyau[2])); //a tuyau : (next point, degrees)
    }
    let mut res = -1;
    if is_solvable(n,a, b, k, &followers_list) {
        let mut next_stages : Vec<(usize, usize)> = vec![(a, 0)]; //a stage : (current point, total temperature reduction)
        let mut i = 0;
        'outer : loop {
            i+=1;
            let current_stages = next_stages;
            next_stages = Vec::new();
            for stage in current_stages {
                for tuyau in &followers_list[stage.0] {
                    let next_point = tuyau.0;
                    let next_reduced_temp = stage.1 + tuyau.1;
                    if next_point == b && next_reduced_temp >= k {
                        res = i;
                        break 'outer;
                    }
                    next_stages.push((next_point, next_reduced_temp));
                }
            }
        }
    }
    println!("{}", res);
}

fn is_solvable(number_of_points : usize, departure: usize, arrival: usize, min_temp : usize, followers_list: &Vec<Vec<(usize, usize)>>) -> bool {
    let mut paths = Vec::<HashSet<usize>>::new();
    let mut visited = vec![false; number_of_points + 1];
    match dfs(departure, arrival, min_temp, 0,HashSet::from([departure]), &followers_list, &mut visited, &mut paths) {
        0 => false, //0 impossible to solve (there is no path from the departure to the arrival)
        1 => true, //1 possible to solve (there is a path from the departure to the arrival with a sufficient temperature reduction)
        _ => { //2 maybe possible to solve (there is a path from the departure to the arrival)
            let mut accessible_from: Vec<Option<HashSet<usize>>> = vec![None; number_of_points + 1];
            for path in paths { //checks if there is any cycles, if there is a cycle the problem is solvable
                for &point in &path {
                    if let Some(accessible) = &accessible_from[point] {
                        if !accessible.is_disjoint(&path) {
                            return true;
                        }
                    } else {
                        let mut accessible = HashSet::new();
                        simple_dfs(point, followers_list, &mut accessible);
                        accessible_from[point] = Some(accessible);
                        if !accessible_from[point].as_ref().unwrap().is_disjoint(&path) {
                            return true;
                        }
                    }
                }
            }
            false
        }
    }
}

fn simple_dfs(point : usize, followers_list : &Vec<Vec<(usize, usize)>>, accessible : &mut HashSet<usize>) {
    for follower in &followers_list[point] {
        if accessible.insert(follower.0) {
            simple_dfs(follower.0, followers_list, accessible);
        }
    }
}

fn dfs(current_point : usize, target : usize, min_temp : usize, current_temp : usize, current_path : HashSet<usize>, followers_list: &Vec<Vec<(usize, usize)>>, visited : &mut Vec<bool>, paths : &mut Vec<HashSet<usize>>) -> usize {
    let mut res = 0;
    visited[current_point] = true;
    for follower in &followers_list[current_point] {
        if !visited[follower.0] {
            let next_temp = current_temp + follower.1;
            if follower.0 == target {
                if next_temp >= min_temp {
                    return 1;
                } else {
                    let mut next_path = current_path.clone();
                    next_path.insert(target);
                    paths.push(next_path);
                    res = 2;
                }
            } else {
                let mut next_path = current_path.clone();
                next_path.insert(follower.0);
                match dfs(follower.0, target, min_temp, next_temp, next_path, &followers_list, visited, paths) {
                    0 => {},
                    1 => return 1,
                    _ => res = 2
                }
            }
        }
    }
    res
}

fn main() {
    let mut buffer = String::new();

    let n = read_line(&mut buffer)
        .parse()
        .expect("invalid `n` parameter");

    let m = read_line(&mut buffer)
        .parse()
        .expect("invalid `m` parameter");

    let k = read_line(&mut buffer)
        .parse()
        .expect("invalid `k` parameter");

    let a = read_line(&mut buffer)
        .parse()
        .expect("invalid `a` parameter");

    let b = read_line(&mut buffer)
        .parse()
        .expect("invalid `b` parameter");

    let tuyaux = (0..m)
        .map(|_| {
            read_line(&mut buffer)
                .split_whitespace()
                .map(str::parse)
                .collect::<Result<_, _>>()
        })
        .collect::<Result<_, _>>()
        .expect("invalid `tuyaux` parameter");

    refroidissement(n, m, k, a, b, tuyaux);
}

fn read_line(buffer: &mut String) -> &str {
    buffer.clear();
    std::io::stdin()
        .read_line(buffer)
        .expect("impossible to read a new line");
    buffer.trim_end()
}