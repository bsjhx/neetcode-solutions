pub fn is_palindrome(s: String) -> bool {
    if s.is_empty() {
        return true;
    }
    let s: Vec<char> = s
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect();

    if s.len() == 1 || s.len() == 0 {
        return true;
    }

    let mut left = 0;
    let mut right = s.len() - 1;

    while left <= right {
        if s[left] != s[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_palindrome_test() {
        assert_eq!(is_palindrome("A man, a plan, a canal: Panama".into()), true);
        assert_eq!(is_palindrome("lol".into()), true);
        assert_eq!(is_palindrome("This is not a palindrome".into()), false);
        assert_eq!(is_palindrome(" ".into()), true);
        assert_eq!(is_palindrome(".".into()), true);
        assert_eq!(is_palindrome("a".into()), true);
        assert_eq!(is_palindrome("0P".into()), false);
    }
}
