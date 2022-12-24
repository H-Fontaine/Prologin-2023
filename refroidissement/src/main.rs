use std::fs::{File, read, read_to_string};

/// * `n` - Le nombre de points
/// * `m` - Le nombre de tuyaux
/// * `k` - Le nombre de degrés minimum de refroidissement
/// * `a` - Le point de départ
/// * `b` - Le point d'arrivée
/// * `tuyaux` - Les tuyaux orientés (point de départ, point d'arrivée, refroidissement)

fn refroidissement(n: usize, m: usize, k: usize, a: usize, b: usize, tuyaux: Vec<Vec<usize>>) {
    let mut liste_adjacence = vec![Vec::<(usize, usize)>::new(); n];
    for tuyau in tuyaux {
        liste_adjacence[tuyau[0]].push((tuyau[1], tuyau[2])); //a tuyau : (next point, degrees)
    }
    let res = -1;
    let mut next_stages : Vec<(usize, usize)> = vec![(a, 0)]; //a stage : (current point, reduced temperature)
    let mut i = 0;
    'outer : while i < 100 {
        i+=1;
        let current_points = next_stages;
        next_stages = Vec::new();
        for stage in current_points {
            for tuyau in &liste_adjacence[stage.0] {
                let next_point = tuyau.0;
                let next_reduced_temp = stage.1 + tuyau.1;
                if next_point == b && next_reduced_temp >= k {
                    break 'outer;
                }
                next_stages.push((next_point, next_reduced_temp));
            }
        }
    }
    println!("{}", res);
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

    /*
    let mut n = 0;
    let mut m = 0;
    let mut k = 0;
    let mut a = 0;
    let mut b = 0;
    let mut tuyaux = Vec::new();

    read_entry(&mut n, &mut m, &mut k, &mut a, &mut b, &mut tuyaux, "entries/entry1.txt");
    */

    refroidissement(n, m, k, a, b, tuyaux);
}

fn read_entry(n: &mut usize, m : &mut usize, k : &mut usize, a : &mut usize, b : &mut usize, tuyaux : &mut Vec<Vec<usize>>, path : &str) {
    let content = read_to_string(path).unwrap();
    let mut lines = content.lines();
    *n = lines.next().unwrap().parse::<i32>().unwrap() as usize;
    *m = lines.next().unwrap().parse::<i32>().unwrap() as usize;
    *k = lines.next().unwrap().parse::<i32>().unwrap() as usize;
    *a = lines.next().unwrap().parse::<i32>().unwrap() as usize;
    *b = lines.next().unwrap().parse::<i32>().unwrap() as usize;
    *tuyaux = (0..*m).map(|_| lines.next().unwrap().split_whitespace().map(str::parse).collect::<Result<_, _>>()).collect::<Result<_, _>>().unwrap();
}

fn read_line(buffer: &mut String) -> &str {
    buffer.clear();
    std::io::stdin()
        .read_line(buffer)
        .expect("impossible to read a new line");
    buffer.trim_end()
}