use regex::Regex;
use std::sync::LazyLock;

static EMAIL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\w[-\w.+]*@([A-Za-z0-9][-A-Za-z0-9]+\.)+[A-Za-z]{2,14}$").unwrap()
});

pub fn is_valid_name(name: &str) -> bool {
    let invalid_chars = [
        '/', '(', ')', '"', '<', '>', '\\', '{', '}', '#', '%', '&', '*', '!', '@', '$',
    ];
    !name.is_empty() & !name.chars().any(|c| invalid_chars.contains(&c))
}

pub fn is_valid_email(email: &str) -> bool {
    !email.is_empty() & EMAIL_REGEX.is_match(email)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_name() {
        assert!(is_valid_name("John Doe"));
        assert!(is_valid_name("小明"));
        assert!(!is_valid_name("小明}"));
        assert!(!is_valid_name("John/Doe"));
        assert!(!is_valid_name("John Doe}"));
    }
}
