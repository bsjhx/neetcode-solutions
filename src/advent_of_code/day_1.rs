use std::fs::read_to_string;
use std::path::Path;

fn calculate() -> i32 {
    let mut sum = 0;
    for line in read_to_string(Path::new("./src/advent_of_code/data_1.txt"))
        .unwrap()
        .lines()
    {
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
    use crate::advent_of_code::day_1::calculate;

    #[test]
    fn calc() {
        let a = calculate();
        println!("{}", a);
    }
}
