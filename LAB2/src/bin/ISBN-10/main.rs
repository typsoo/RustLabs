use std::io;

fn main() {
    match get_users_input() {
        Some(isbn) => {
            if is_valid_isbn10(&isbn) {
                println!("Podany ciąg jest prawidłowym numerem ISBN-10.");
            } else {
                println!("Podany ciąg NIE jest prawidłowym numerem ISBN-10.");
            }
        }
        None => {
            println!(
                "Błąd: Nieprawidłowa długość. ISBN-10 musi mieć dokładnie 10 znaków (nie licząc myślników)."
            );
        }
    }
}

fn get_users_input() -> Option<String> {
    println!("Please ISBN-10 number:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let cleaned_input: String = input.trim().chars().filter(|c| *c != '-').collect();

    if cleaned_input.len() != 10 {
        return None;
    }

    Some(cleaned_input)
}

fn is_valid_isbn10(isbn: &str) -> bool {
    let mut sum = 0;

    for (i, c) in isbn.chars().enumerate() {
        let w = 10 - i as u32;

        let value = match c {
            '0'..='9' => c.to_digit(10).unwrap(),
            'x' | 'X' if i == 9 => 10,
            _ => return false,
        };

        sum += w * value
    }

    sum % 11 == 0
}
