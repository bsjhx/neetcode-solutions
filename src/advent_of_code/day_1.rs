use std::fs::read_to_string;
use std::path::Path;
use std::str::Lines;

pub fn calculate_calibration_document(path: &str) -> i32 {
    let lines = read_to_string(Path::new(path)).unwrap();
    let lines = lines.lines();
    calculate(lines)
}

fn calculate(lines: Lines) -> i32 {
    let mut sum = 0;
    for line in lines.into_iter() {
        sum = sum + find_first_and_last_numbers(line);
    }

    sum
}

fn find_first_and_last_numbers(s: &str) -> i32 {
    if s.is_empty() {
        return 0;
    }
    let s: Vec<char> = s
        .to_lowercase()
        .chars()
        .filter(|c| ('0'..='9').contains(c))
        .collect();

    let mut result = s[0].to_string();
    result.push(s[s.len() - 1]);

    result.parse::<i32>().unwrap()
}

#[cfg(test)]
mod test {
    use crate::day_1::calculate;

    #[test]
    fn test_calc() {
        let lines = "1ala5\ngfh5gf8hfg6fg".lines();
        assert_eq!(calculate(lines), 15 + 56);

        let lines = "dsfsd9fsdfsd\n1ala5\ngfh5gfhfg6fg\n1".lines();
        assert_eq!(calculate(lines), 99 + 15 + 56 + 11);
    }
}
