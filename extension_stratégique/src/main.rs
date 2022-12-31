
/// description d'une position dans le plan spatio-temporel
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Position {
    /// la coordonnée x dans le plan spatio-temporel
    x: i32,
    /// la coordonnée y dans le plan spatio-temporel
    y: i32,
}

/// * `d` - le rayon de l'espace-temps connu
/// * `n` - le nombre de points de contrôle existants (sans compter celui sur lequel vous vous situez actuellement)

/// * `points_de_controles` - la liste des coordonnées points de contrôle existants
fn aretes_minimales(d: i32, n: i32, points_de_controles: Vec<Position>) {
}

fn main() {
    let mut buffer = String::new();

    let d = read_line(&mut buffer)
        .parse()
        .expect("invalid `D` parameter");

    let n = read_line(&mut buffer)
        .parse()
        .expect("invalid `N` parameter");

    let points_de_controles = (0..n)
        .map(|_| read_line(&mut buffer).parse())
        .collect::<Result<_, _>>()
        .expect("invalid `points_de_controles` parameter");

    aretes_minimales(d, n, points_de_controles);
}

fn read_line(buffer: &mut String) -> &str {
    buffer.clear();
    std::io::stdin()
        .read_line(buffer)
        .expect("impossible to read a new line");
    buffer.trim_end()
}

impl std::str::FromStr for Position {
    type Err = Box<dyn std::error::Error>;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut line = line.split_whitespace();
        Ok(Self {
            x: line.next().ok_or("missing `x`")?.parse()?,
            y: line.next().ok_or("missing `y`")?.parse()?,
        })
    }
}