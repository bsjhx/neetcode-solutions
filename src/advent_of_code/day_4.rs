use std::fs::read_to_string;
use std::path::Path;
use std::str::Lines;

pub fn calculate_card(path: &str) -> i32 {
    let lines = read_to_string(Path::new(path)).unwrap();
    let lines = lines.lines();
    calculate(lines)
}

// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53

fn calculate(lines: Lines) -> i32 {
    let mut sum: i32 = 0;

    for line in lines.into_iter() {
        let both_numbers = line.split(":").collect::<Vec<&str>>()[1];
        let both_numbers = both_numbers.split("|").collect::<Vec<&str>>();
        let winning_numbers = both_numbers[0];
        let played_numbers = both_numbers[1];

        let winning_numbers = parse_to_numbers(winning_numbers);
        let played_numbers = parse_to_numbers(played_numbers);

        let mut found = winning_numbers
            .iter()
            .filter(|n| played_numbers.contains(n))
            .count();
        if found != 0 {
            found = found - 1;
            sum += 2_i32.pow(found as u32);
        }
    }

    sum
}

fn parse_to_numbers(played_numbers: &str) -> Vec<i32> {
    played_numbers
        .split(" ")
        .map(|s| s.parse::<i32>())
        .filter(|p| p.is_ok())
        .map(|s| s.unwrap())
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use crate::day_4::calculate;

    const TEXT_LONGER: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn calculate_card() {
        let result = calculate("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".lines());
        assert_eq!(result, 8);

        let result = calculate("Card 1: 41 48 83 86 17 | 17".lines());
        assert_eq!(result, 1);

        let result = calculate(TEXT_LONGER.lines());
        assert_eq!(result, 13);
    }
}
