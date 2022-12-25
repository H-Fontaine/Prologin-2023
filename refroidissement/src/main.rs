use std::cmp::min;
use std::collections::HashSet;

/// * `n` - Le nombre de points
/// * `m` - Le nombre de tuyaux
/// * `k` - Le nombre de degrés minimum de refroidissement
/// * `a` - Le point de départ
/// * `b` - Le point d'arrivée
/// * `tuyaux` - Les tuyaux orientés (point de départ, point d'arrivée, refroidissement)

fn refroidissement(n: usize, _m: usize, k: usize, a: usize, b: usize, tuyaux: Vec<Vec<usize>>) {
    let mut adjacency_lists = vec![vec![Vec::<(usize, usize)>::new(); n + 1]; n + 1];
    let mut adjacency_list_reversed = vec![Vec::<(usize, usize)>::new(); n + 1];
    let mut mins = vec![n+1; n+1];
    let mut circuits = vec![Vec::<(usize, usize)>::new(); n+1];
    for tuyau in tuyaux {
        adjacency_list_reversed[tuyau[1]].push((tuyau[0], tuyau[2])); //a tuyau : (next point, degrees reduction)

        let max = min(tuyau[0], tuyau[1]);
        let mut i = 1;
        while i <= max {
            adjacency_lists[i][tuyau[0]].push((tuyau[1], tuyau[2]));
            if mins[i] > max {
                mins[i] = max;
            }
            i+=1;
        }
    }

    let (distances_to_begin, costs_to_begin) = dijkstra(a, &adjacency_lists[1], n);
    let (distances_to_end, costs_to_end) = dijkstra(b, &adjacency_list_reversed, n);

    let shortest_path_through_points : Vec<Option<(usize, usize)>> = distances_to_begin.into_iter().zip(distances_to_end.into_iter()).zip(costs_to_begin.into_iter()).zip(costs_to_end.into_iter()).map(|(((distance_to_begin,distance_to_end),cost_to_begin),cost_to_end)| {
        if !distance_to_begin.is_none() && !distance_to_end.is_none() {
            Some((distance_to_begin.unwrap() + distance_to_end.unwrap(), cost_to_begin.unwrap() + cost_to_end.unwrap()))
        } else {
            None
        }
    }).collect();

    let mut blocked = vec![false; n + 1];
    let mut blocked_sets = vec![HashSet::new(); n + 1];
    let mut stack = HashSet::new();
    let mut s = 1;
    while s < n {
        if mins[s] < n + 1 {
            for i in s..(n+1) {
                blocked[i] = false;
                blocked_sets[i] = HashSet::new();
            }
            circuit(s, 0, s, &mut blocked, &adjacency_lists, &mut stack, 0, &mut blocked_sets, &mut circuits);
            s += 1;
        } else {
            s = n;
        }
    }
    println!("{:?}", circuits);
}

fn unblock(u : usize, blocked : &mut Vec<bool>, blocked_sets : &mut Vec<HashSet<usize>>) {
    blocked[u] = false;
    for w in blocked_sets[u].clone() {
        blocked_sets[u].remove(&w);
        if blocked[w] {
            unblock(w, blocked, blocked_sets);
        }
    }
}

fn circuit(v : usize, cost : usize, s: usize, blocked : &mut Vec<bool>, adjacency_lists : &Vec<Vec<Vec<(usize, usize)>>>, stack : &mut HashSet<usize>, mut stack_cost : usize, blocked_sets : &mut Vec<HashSet<usize>>, circuits : &mut Vec<Vec<(usize, usize)>>) -> bool {
    let mut f = false;
    stack.insert(v);
    stack_cost += cost;
    blocked[v] = true;
    for &w in &adjacency_lists[s][v] {
        if w.0 == s {
            stack_cost += w.1;
            for &vertex in &*stack {
                circuits[vertex].push((stack.len(), stack_cost));
            }
            stack_cost -= w.1;
            f = true;
        } else if !blocked[w.0] {
            if circuit(w.0, w.1, s, blocked, adjacency_lists, stack, stack_cost, blocked_sets, circuits) {
                f = true
            }
        }
    }
    if f {
        unblock(v, blocked, blocked_sets);
    } else {
        for &w in &adjacency_lists[s][v] {
            blocked_sets[w.0].insert(v);
        }
    }
    stack.remove(&v);
    f
}

fn dijkstra(start_vertex : usize, followers_list : &Vec<Vec<(usize, usize)>>, number_of_vertex : usize) -> (Vec<Option<usize>>, Vec<Option<usize>>) {
    let mut min_distance_to_begin = vec![None; number_of_vertex + 1];
    let mut min_cost_to_begin = vec![None; number_of_vertex + 1];
    let mut processed_vertex = vec![false; number_of_vertex +1];
    min_distance_to_begin[start_vertex] = Some(0);
    min_cost_to_begin[start_vertex] = Some(0);
    processed_vertex[start_vertex] = true;
    let mut pivot_vertex = start_vertex;
    let mut old_pivot_vertex = None;
    while old_pivot_vertex.is_none() || pivot_vertex != old_pivot_vertex.unwrap() {
        for &succ_vertex in &followers_list[pivot_vertex] {
            if !processed_vertex[succ_vertex.0] {
                let distance = min_distance_to_begin[pivot_vertex].unwrap() + 1;
                if min_distance_to_begin[succ_vertex.0].is_none() || distance < min_distance_to_begin[succ_vertex.0].unwrap() {
                    min_distance_to_begin[succ_vertex.0] = Some(distance);
                    min_cost_to_begin[succ_vertex.0] = Some(min_cost_to_begin[pivot_vertex].unwrap() + succ_vertex.1);
                }
            }
        }
        let mut min_distance = None;
        old_pivot_vertex = Some(pivot_vertex);
        for vertex in 1..(number_of_vertex + 1) {
            if !processed_vertex[vertex] && !min_cost_to_begin[vertex].is_none() {
                if min_distance.is_none() || min_distance.unwrap() > min_distance_to_begin[vertex].unwrap() {
                    min_distance = Some(min_distance_to_begin[vertex].unwrap());
                    pivot_vertex = vertex;
                }
            }
        }
        processed_vertex[pivot_vertex] = true;
    }
    (min_distance_to_begin, min_cost_to_begin)
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