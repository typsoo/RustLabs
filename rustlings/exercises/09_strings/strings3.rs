fn trim_me(input: &str) -> String {
    let bytes = input.as_bytes();

    if bytes.is_empty() {
        return String::from("");
    }

    let mut start = 0;
    let mut end = bytes.len() - 1;

    while start <= end && bytes[start] == b' ' {
        start += 1;
    }

    while end > start && bytes[end] == b' ' {
        end -= 1;
    }

    input[start..=end].to_string()
} 

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There are multiple ways to do this.
    input.to_owned() + " world!"
}

fn replace_me(input: &str) -> String {
    let n = input.len();
    if n < 4 {
        return String::from("");
    }
    let bytes = input.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b'c' {
            if i + 3 < n && &input[i..i + 4] == "cars" {
                return format!("{}{}{}", &input[..i], "balloons", &input[i + 4..]);
            }
        }
    }
    "".to_string()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
        assert_eq!(trim_me("Hi!"), "Hi!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool",
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons",
        );
    }
}
