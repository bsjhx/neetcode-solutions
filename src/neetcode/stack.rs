pub fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = vec![];

    for c in s.chars() {
        if c == '(' || c == '[' || c == '{' {
            stack.push(c);
        } else {
            let last_from_stack = stack.pop().unwrap_or_default();
            if c == ')' && last_from_stack != '(' {
                return false;
            } else if c == ']' && last_from_stack != '[' {
                return false;
            } else if c == '}' && last_from_stack != '{' {
                return false;
            }
        }
    }

    stack.is_empty()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(is_valid("()".into()), true);
        assert_eq!(is_valid("(]".into()), false);
        assert_eq!(is_valid("({)}".into()), false);
        assert_eq!(is_valid("({})".into()), true);
        assert_eq!(is_valid("()[]{}".into()), true);
    }
}
