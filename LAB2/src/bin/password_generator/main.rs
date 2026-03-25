use rand::Rng;
use std::char;

fn create_pass_str(charset: &[&str]) -> String {
    let mut all_signs = String::new();
    for &item in charset.iter() {
        all_signs += match item {
            "lowercase" => "qwertyuiopasdfghjklzxcvbnm",
            "uppercase" => "QWERTYUIOPASDFGHJKLZXCVBNM",
            "digits" => "1234567890",
            "specials" => "!@#$%^&*()_+}{?.,",
            _ => "",
        }
    }
    if all_signs.len() == 0 {
        return "qwertyuiopasdfghjklzxcvbnm1234567890!@#$%^&*()_+:;QWERTYUIOPASDFGHJKLZXCVBNM"
            .to_string();
    }
    all_signs
}

fn generate_password(n: u8, charset: &[&str]) -> String {
    let all_signs_owned = create_pass_str(charset);
    let all_signs = all_signs_owned.as_bytes();
    let mut password = String::new();

    let mut rng = rand::rng();
    for _ in 0..n {
        let rand_index = rng.random_range(0..all_signs.len());
        password.push(all_signs[rand_index] as char);
    }

    password
}

fn main() {
    println!(
        "Password: {}",
        generate_password(12, &["lowercase", "specials"])
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_len_test() {
        assert_eq!(5, generate_password(5, &[]).len());
        assert_eq!(0, generate_password(0, &[]).len());
    }

    #[test]
    fn test_password_contains_only_allowed_chars() {
        let config = ["lowercase", "digits"];

        let password_length = 100;
        let password = generate_password(password_length, &config);

        let allowed_bytes = b"qwertyuiopasdfghjklzxcvbnm1234567890";

        assert_eq!(password.len(), password_length as usize);

        for ch in password.chars() {
            assert!(
                allowed_bytes.contains(&(ch as u8)),
                "Test failed: Found invalid character '{}' in password",
                ch
            );
        }
    }
}
