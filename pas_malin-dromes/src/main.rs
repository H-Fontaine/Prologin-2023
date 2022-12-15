/// * `n` - Le nombre de mots de passe contenus dans le fichier de mots de passe de Raphaël
/// * `mots` - La liste des mots de passe à décoder
fn nb_pas_malin_drome(n: i32, mots: Vec<String>) {
    let mut res =0;
    for mot in mots {
        let mut upper = Vec::with_capacity(mot.len());
        let mut lower = Vec::with_capacity(mot.len());
        let mut number = Vec::with_capacity(mot.len());
        for letter in mot.chars() {
            if letter.is_numeric() {
                number.push(letter);
            } else if letter.is_uppercase() {
                upper.push(letter);
            } else if letter.is_lowercase() {
                lower.push(letter);
            }
        }
        if is_palindrome(upper) && is_palindrome(lower) && is_palindrome(number) {
            res += 1;
        }
    }
    println!("{}", res);
}

fn is_palindrome(mot : Vec<char>) -> bool {
    let mut is_palindrom = true;
    if mot.len() != 0 {
        let mut left_indice = 0;
        let mut right_indice = mot.len() - 1;
        while is_palindrom && left_indice < right_indice {
            is_palindrom = mot[left_indice] == mot[right_indice];
            left_indice += 1;
            right_indice -= 1;
        }
    }
    is_palindrom
}

fn main() {
    let mut buffer = String::new();

    let n = read_line(&mut buffer)
        .parse()
        .expect("invalid `N` parameter");

    let mots = (0..n)
        .map(|_| read_line(&mut buffer).to_string())
        .collect();

    nb_pas_malin_drome(n, mots);
}

fn read_line(buffer: &mut String) -> &str {
    buffer.clear();
    std::io::stdin()
        .read_line(buffer)
        .expect("impossible to read a new line");
    buffer.trim_end()
}