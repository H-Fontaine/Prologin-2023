use std::cmp::{min, max};
use std::collections::HashSet;

/// * `n` - Le nombre de points
/// * `m` - Le nombre de tuyaux
/// * `k` - Le nombre de degrés minimum de refroidissement
/// * `a` - Le point de départ
/// * `b` - Le point d'arrivée
/// * `tuyaux` - Les tuyaux orientés (point de départ, point d'arrivée, refroidissement)

fn refroidissement(n: usize, m: usize, k: i64, a: usize, b: usize, tuyaux: Vec<Vec<i64>>) {
    let mut adjacency_lists = vec![vec![Vec::<(usize, i64)>::new(); n + 1]; n + 1];
    let mut adjacency_list_reversed = vec![Vec::<(usize, i64)>::new(); n + 1];
    let mut mins = vec![n+1; n+1];
    let mut one_cycle = Vec::new();
    for tuyau in tuyaux {
        adjacency_list_reversed[tuyau[1] as usize].push((tuyau[0] as usize, tuyau[2])); //a tuyau : (next point, degrees reduction)

        let max = min(tuyau[0], tuyau[1]) as usize;
        let mut i : usize = 1;
        while i <= max {
            adjacency_lists[i][tuyau[0] as usize].push((tuyau[1] as usize, tuyau[2]));
            if mins[i] > max {
                mins[i] = max;
            }
            i+=1;
        }

        if tuyau[0] == tuyau[1] {
            one_cycle.push((HashSet::from([tuyau[0] as usize]), tuyau[2]));
        }
    }

    /*
    match get_shortest_path_through_points(a, b, n, &adjacency_lists[1], &adjacency_list_reversed) {
        None => {
            println!("{}", -1);
            return
        },
        Some(paths) => {
            let mut min_distance = max(k, n as i64);
            let mut solved = false;
            for path in &paths[1..] {
                if !path.is_none() && path.as_ref().unwrap().1 >= k && path.as_ref().unwrap().0 as i64 <= min_distance {
                    min_distance = path.as_ref().unwrap().0 as i64;
                    solved = true;
                }
            }
            if solved {
                println!("{}", min_distance);
                return
            }

            let mut circuits = find_all_circuits(n, &adjacency_lists, &mins);
            circuits.extend(one_cycle);
            let mut circuit_pool : HashSet<usize> = HashSet::new();
            for (circuit, _cost) in &circuits {
                circuit_pool.extend(circuit);
            }


            let mut i: usize = 0;
            for path in paths { //a path (distance, cost)
                if !path.is_none() && circuit_pool.contains(&i) {
                    let to_fill = k - path.as_ref().unwrap().1;
                    min_distance = find_min_distance(path.as_ref().unwrap().0 as i64, path.unwrap().2, &circuits, to_fill, min_distance);
                    solved = true;
                }
                i += 1;
            }
            if solved {
                println!("{}", min_distance);
                return
            }
            println!("{}", -1);
        }
    }
    */



    match get_all_paths(a, b, &adjacency_lists[1]) {
        None => {
            println!("{}", -1);
            return
        }
        Some(paths) => {
            let mut minimal_distance = 2 * k * (n as i64) * (m as i64);
            let mut solved = false;
            let mut circuits = find_all_circuits(n, &adjacency_lists, &mins);
            circuits.extend(one_cycle.into_iter());
            let mut circuit_pool : HashSet<usize> = HashSet::new();
            for (circuit, _cost) in &circuits {
                circuit_pool.extend(circuit);
            }

            for (path, path_cost) in paths {
                if path_cost >= k {
                    if minimal_distance > path_cost {
                        minimal_distance = path_cost;
                        solved = true
                    }
                } else {
                    if !path.is_disjoint(&circuit_pool) {
                        let mut distance = path.len() as i64;
                        if a != b {
                            distance -= 1;
                        }
                        minimal_distance = find_min_distance(distance, path, &circuits, k - path_cost, minimal_distance);
                        solved = true;
                    }
                }
            }
            if solved {
                println!("{}", minimal_distance);
            } else {
                println!("{}", -1);
            }
        }
    }
}

fn search_paths(start_vertex : usize, end_vertex : usize, cost : i64, adjacency_list : &Vec<Vec<(usize, i64)>>, mut current_path : HashSet<usize>, paths : &mut Vec<(HashSet<usize>, i64)>) {
    current_path.insert(start_vertex);
    for link in &adjacency_list[start_vertex] {
        if link.0 == end_vertex {
            current_path.insert(end_vertex);
            paths.push((current_path, cost + link.1));
            break
        }
        if !current_path.contains(&link.0) {
            search_paths(link.0, end_vertex, cost + link.1, adjacency_list, current_path.clone(), paths);
        }
    }
}

fn get_all_paths(start_vertex : usize, end_vertex : usize, adjacency_list : &Vec<Vec<(usize, i64)>>) -> Option<Vec<(HashSet<usize>, i64)>> {
    let mut res = Vec::new();
    let current_path = HashSet::new();
    search_paths(start_vertex, end_vertex, 0, adjacency_list, current_path, &mut res);
    if res.len() >= 1 {
        Some(res)
    } else {
        None
    }
}


/*
Trouve recursivement la distance minimale possible pour avoir le refroidissement necessaire
*/
fn find_min_distance(distance : i64, available_vertices : HashSet<usize> , circuits : &Vec<(HashSet<usize>, i64)>, to_fill : i64, mut min_distance : i64) -> i64 {
    for vertex in &available_vertices {
        for (circuit, cost) in circuits {
            if circuit.contains(vertex) {
                if min_distance > distance + circuit.len() as i64 {
                    if *cost >= to_fill {
                        min_distance = distance + circuit.len() as i64;
                    } else {
                        let mut new_available_vertices = available_vertices.clone();
                        new_available_vertices.extend(circuit);
                        min_distance = find_min_distance(distance + circuit.len() as i64, new_available_vertices, circuits, to_fill - cost, min_distance);
                    }
                }
            }
        }
    }
    min_distance
}


fn get_shortest_path_through_points(start_vertex : usize, end_vertex : usize, number_of_vertex : usize, adjacency_list : &Vec<Vec<(usize, i64)>>, adjacency_list_reversed : &Vec<Vec<(usize, i64)>>) -> Option<Vec<Option<(usize, i64, HashSet<usize>)>>> {
    let (distances_to_begin, costs_to_begin, paths_from_begin) = dijkstra(start_vertex, adjacency_list, number_of_vertex);
    let (distances_to_end, costs_to_end, paths_from_end) = dijkstra(end_vertex, adjacency_list_reversed, number_of_vertex);
    let mut count = 0;
    let paths = distances_to_begin.into_iter().zip(distances_to_end.into_iter()).zip(costs_to_begin.into_iter()).zip(costs_to_end.into_iter()).zip(paths_from_begin.into_iter()).zip(paths_from_end.into_iter()).map(|(((((distance_to_begin,distance_to_end),cost_to_begin),cost_to_end), mut path_from_begin), path_from_end)| {
        if !distance_to_begin.is_none() && !distance_to_end.is_none() {
            path_from_begin.as_mut().unwrap().extend(path_from_end.unwrap());
            Some((distance_to_begin.unwrap() + distance_to_end.unwrap(), cost_to_begin.unwrap() + cost_to_end.unwrap(), path_from_begin.unwrap()))
        } else {
            count += 1;
            None
        }
    }).collect();
    if count == number_of_vertex + 1 {
        None
    } else {
        Some(paths)
    }
}

/*
Algorithme pour resoudre le probleme du sac a dos
Inutile car ne permet pas de prendre en compte les nouveaux circuits accessible après utilisation d'un autre
*/
fn knapsack(minimal_value : usize, max_weight : usize, circuits : &Vec<(usize, usize)>) -> usize {
    let mut costs = vec![0; max_weight + 1];

    for i in 0..(max_weight + 1) {
        for j in 0..circuits.len() {
            if circuits[j].0 <= i {
                costs[i] = max(costs[i], costs[i - circuits[j].0] + circuits[j].1);
            }
            if costs[i] >= minimal_value {
                return i
            }
        }
    }
    return max_weight
}

/*
Algorithme de Donald B. Johnson pour trouver les cicuits elementaires dans un graphe, utilise les fonctions circuit & unblock
Lien : https://www.cs.tufts.edu/comp/150GA/homeworks/hw1/Johnson%2075.PDF
Renvoie :
    Vec<(HashSet<usize>, usize)> : un tableau contenant les differents circuits elementaires : (HastSet<usize> : les sommets qui composent le cycle, usize : la somme des valuations des arretes traversee dans le cycle)
*/
fn find_all_circuits(number_of_points : usize, adjacency_lists : &Vec<Vec<Vec<(usize, i64)>>>, mins : &Vec<usize>) -> Vec<(HashSet<usize>, i64)> {
    let mut circuits = Vec::new();
    let mut blocked = vec![false; number_of_points + 1];
    let mut blocked_sets = vec![HashSet::new(); number_of_points + 1];
    let mut stack = HashSet::new();
    let mut s = 1;
    while s < number_of_points {
        if mins[s] < number_of_points + 1 {
            for i in s..(number_of_points+1) {
                blocked[i] = false;
                blocked_sets[i] = HashSet::new();
            }
            circuit(s, 0, s, &mut blocked, &adjacency_lists, &mut stack, 0, &mut blocked_sets, &mut circuits);
            s += 1;
        } else {
            s = number_of_points;
        }
    }
    circuits
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

fn circuit(v : usize, cost : i64, s: usize, blocked : &mut Vec<bool>, adjacency_lists : &Vec<Vec<Vec<(usize, i64)>>>, stack : &mut HashSet<usize>, mut stack_cost : i64, blocked_sets : &mut Vec<HashSet<usize>>, circuits : &mut Vec<(HashSet<usize>, i64)>) -> bool {
    let mut f = false;
    stack.insert(v);
    stack_cost += cost;
    blocked[v] = true;
    for &w in &adjacency_lists[s][v] {
        if w.0 == s {
            stack_cost += w.1;
            circuits.push((stack.clone(), stack_cost));
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

/*
Algorithme de plus court chemin
Renvoie :
    (Vec<Option<usize>>, Vec<Option<usize>>) : tableau contenant la distance à l'origine (le nombre d'arrete traversees pour atteindre le sommet i), même chose mais cette fois cette la somme des valuations des arretes traversees
*/
fn dijkstra(start_vertex : usize, followers_list : &Vec<Vec<(usize, i64)>>, number_of_vertex : usize) -> (Vec<Option<usize>>, Vec<Option<i64>>, Vec<Option<HashSet<usize>>>) {
    let mut min_distance_to_begin = vec![None; number_of_vertex + 1];
    let mut min_cost_to_begin = vec![None; number_of_vertex + 1];
    let mut paths = vec![None; number_of_vertex + 1];
    let mut parents = vec![None; number_of_vertex + 1];
    let mut processed_vertex = vec![false; number_of_vertex +1];
    paths[start_vertex] = Some(HashSet::from([start_vertex]));
    min_distance_to_begin[start_vertex] = Some(0);
    min_cost_to_begin[start_vertex] = Some(0);
    processed_vertex[start_vertex] = true;
    let mut pivot_vertex = start_vertex;
    let mut old_pivot_vertex = None;
    while old_pivot_vertex.is_none() || pivot_vertex != old_pivot_vertex.unwrap() {
        for &succ_vertex in &followers_list[pivot_vertex] {
            if !processed_vertex[succ_vertex.0] || succ_vertex.0 == start_vertex {
                let distance = min_distance_to_begin[pivot_vertex].unwrap() + 1;
                if min_distance_to_begin[succ_vertex.0].is_none() || distance < min_distance_to_begin[succ_vertex.0].unwrap() {
                    min_distance_to_begin[succ_vertex.0] = Some(distance);
                    min_cost_to_begin[succ_vertex.0] = Some(min_cost_to_begin[pivot_vertex].unwrap() + succ_vertex.1);
                    parents[succ_vertex.0] = Some(pivot_vertex);
                }
            }
        }
        let mut min_distance = None;
        old_pivot_vertex = Some(pivot_vertex);
        for vertex in 1..(number_of_vertex + 1) {
            if !processed_vertex[vertex] && !min_cost_to_begin[vertex].is_none() {
                if min_distance.is_none() || min_distance.unwrap() > min_distance_to_begin[vertex].unwrap() {
                    min_distance = Some(min_distance_to_begin[vertex].unwrap());
                    let mut path = paths[pivot_vertex].clone().unwrap();
                    path.insert(vertex);
                    paths[vertex] = Some(path);
                    pivot_vertex = vertex;
                }
            }
        }
        processed_vertex[pivot_vertex] = true;
    }
    paths[start_vertex].as_mut().unwrap().remove(&start_vertex);
    (min_distance_to_begin, min_cost_to_begin, paths)
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