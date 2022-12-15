use std::cell::RefCell;
use std::rc::Rc;

/// * `n` - le nombre de cinémas
/// * `redirection` - le lieu de redirection de chaque cinéma
fn trajets_retour(n: usize, redirection: Vec<usize>) {
    let mut previous : Vec<Option<usize>> = vec![None; n];
    let mut distances : Vec<Option<Rc<RefCell<usize>>>> = vec![None; n];
    for i in 0..n {
        if distances[i].is_none() {
            let mut j = i;

            //until we reach an already visited cinema
            while previous[redirection[j]].is_none() {
                previous[redirection[j]] = Some(j);
                j = redirection[j];
            }


            match distances[redirection[j]].clone() {
                Some(distance) => { //if we get to an already visited of which we already know the distance cinema we can calculate the distance easily
                    let mut k = j;
                    let mut new_distance = *distance.borrow() + 1;
                    distances[k] = Some(Rc::new(RefCell::new(new_distance)));
                    while let Some(prev) = previous[k] {
                        new_distance += 1;
                        distances[prev] = Some(Rc::new(RefCell::new(new_distance)));
                        k = prev;
                    }
                }
                None => { //if we get to an already visited cinema but of which we don't know the distance
                    let knot = redirection[j];
                    let mut k = knot;
                    let mut cycle_size = 1;
                    let boxed_cycle_size = Rc::new(RefCell::new(0));
                    while redirection[k] != knot {
                        cycle_size += 1;
                        distances[k] = Some(boxed_cycle_size.clone());
                        k = redirection[k];
                    }
                    distances[k] = Some(boxed_cycle_size.clone());
                    *boxed_cycle_size.borrow_mut() = cycle_size;

                    k = knot; //Here we are at the same point if we had gotten to an already visited cinema of which we already knew the distance
                    let mut new_distance = cycle_size;
                    while let Some(prev) = previous[k] {
                        new_distance += 1;
                        distances[prev] = Some(Rc::new(RefCell::new(new_distance)));
                        k = prev;
                    }
                }
            }
        }
        print!("{} ", distances[i].clone().unwrap().borrow());
    }
}

fn main() {
    let mut buffer = String::new();

    let n = read_line(&mut buffer)
        .parse()
        .expect("invalid `N` parameter");

    let redirection: Vec<usize> = read_line(&mut buffer)
        .split_whitespace()
        .map(|a| a.parse::<usize>().expect("invalid `redirection` parameter") - 1)
        .collect();

    trajets_retour(n, redirection);
}

fn read_line(buffer: &mut String) -> &str {
    buffer.clear();
    std::io::stdin()
        .read_line(buffer)
        .expect("impossible to read a new line");
    buffer.trim_end()
}