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
fn aretes_minimales(d: i32, n: i32, mut points_de_controles: Vec<Position>) {
    points_de_controles.push(Position{x:0,y:0});
    if points_de_controles.len() == 2 {
        println!("{}", 2);
        return;
    }
    if points_de_controles.len() == 3 && are_aligned(points_de_controles[0], points_de_controles[1], points_de_controles[2]) {
        println!("{}", 2);
        return;
    }
    println!("{:?}", detect_shape(&points_de_controles));
}

fn angle(previous_point : Position, current_point : Position, next_point : Position) -> f64 {
    let line = ((current_point.x - previous_point.x) as f64, (current_point.y - previous_point.y) as f64);
    let vector = ((next_point.x - current_point.x) as f64 , (next_point.y - current_point.y) as f64);

    let unsigned_angle = ((line.0 * vector.0 + line.1 * vector.1) / ((line.0.powi(2) + line.1.powi(2)) * (vector.0.powi(2) + vector.1.powi(2))).sqrt()).acos();
    if line.0 * vector.1 - line.1 * vector.0 <= 0f64 {
        unsigned_angle
    } else {
        - unsigned_angle
    }
}

fn detect_shape(points : &Vec<Position>) -> Vec<Position> {
    let mut visited = vec![-1; points.len()];
    let direction_point = Position {x : 0, y : -1};
    let mut res = Vec::new();

    let mut points_order_list = Vec::new();

    visited[points.len() - 1] = 0;
    let mut count = 1;
    points_order_list.push(points.len() - 1);
    let mut min_angle = angle(direction_point, points[points.len() - 1], points[0]);
    let mut next_angle_id = 0;
    let mut previous_angle_id = points.len() - 1;
    for i in 1..(points.len() - 1) {
        let calculated_angle = angle(direction_point, points[points.len() - 1], points[i]);
        if calculated_angle.signum() == min_angle.signum() {
            if calculated_angle < min_angle {
                min_angle = calculated_angle;
                next_angle_id = i;
            }
        } else if calculated_angle >= 0f64 {
            min_angle = calculated_angle;
            next_angle_id = i;
        }
    }

    while visited[next_angle_id] == -1 {
        visited[next_angle_id] = count;
        count += 1;
        points_order_list.push(next_angle_id);

        let angle_id = next_angle_id;
        next_angle_id = points.len() - 1;
        let mut min_angle = angle(points[previous_angle_id], points[angle_id], points[points.len() -1]);
        for i in 0..(points.len() - 1) {
            if i != angle_id && i != previous_angle_id {
                let calculated_angle = angle(points[previous_angle_id], points[angle_id], points[i]);
                if calculated_angle.signum() == min_angle.signum() {
                    if calculated_angle < min_angle {
                        min_angle = calculated_angle;
                        next_angle_id = i;
                    }
                } else if calculated_angle >= 0f64 {
                    min_angle = calculated_angle;
                    next_angle_id = i;
                }
            }
        }
        previous_angle_id = angle_id;
    }

    let start_id = visited[next_angle_id] as usize;
    let end_id = points_order_list.len() - 1;
    if !are_aligned(points[points_order_list[start_id]], points[points_order_list[start_id +1]], points[points_order_list[end_id]]) {
        res.push(points[points_order_list[start_id]]);
    }
    for i in (start_id + 1 as usize)..end_id {
        if !are_aligned(points[points_order_list[i - 1]], points[points_order_list[i]], points[points_order_list[i + 1]]) {
            res.push(points[points_order_list[i]]);
        }
    }
    if !are_aligned(points[points_order_list[start_id]], points[points_order_list[end_id - 1]], points[points_order_list[end_id]]) {
        res.push(points[points_order_list[end_id]]);
    }
    res
}

fn are_aligned(a : Position, b : Position, c : Position) -> bool {
    (b.x - a.x) * (c.y - a.y) == (b.y - a.y) * (c.x - a.x)
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