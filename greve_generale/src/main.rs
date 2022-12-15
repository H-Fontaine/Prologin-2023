/// * `n` - le nombre de cinémas
/// * `redirection` - le lieu de redirection de chaque cinéma
fn trajets_retour(n: i32, redirection: Vec<i32>) {
    for int in redirection {
        println!("{}", int);
    }
}

fn main() {
    let mut buffer = String::new();

    let n = read_line(&mut buffer)
        .parse()
        .expect("invalid `N` parameter");

    let redirection: Vec<i32> = read_line(&mut buffer)
        .split_whitespace()
        .map(|a| a.parse::<i32>().expect("invalid `redirection` parameter") - 1)
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