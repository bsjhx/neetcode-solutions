use std::fs::read_to_string;
use std::path::Path;
use std::str::Lines;

pub fn calculate_gear(path: &str) -> i32 {
    let lines = read_to_string(Path::new(path)).unwrap();
    let lines = lines.lines();
    calculate(lines)
}

fn calculate(lines: Lines) -> i32 {
    let mut sum = 0;

    let mut iterator = lines.into_iter();
    let mut prev_line: Vec<char> = vec![];
    let mut curr_line: Vec<char> = iterator.next().unwrap().chars().collect();
    let mut next_line: Vec<char> = iterator.next().unwrap().chars().collect();

    loop {
        let mut temp = "".to_string();
        for (i, c) in curr_line.clone().into_iter().enumerate() {
            if c.is_numeric() {
                temp.push(c);
            } else if !temp.is_empty() {
                if has_line_special_signs(i, temp.len(), prev_line.clone())
                    || has_line_special_signs(i, temp.len(), curr_line.clone())
                    || has_line_special_signs(i, temp.len(), next_line.clone())
                {
                    sum += temp.parse::<i32>().unwrap();
                    temp = "".to_string();
                }
            }
        }

        if temp != "" {}

        prev_line = curr_line.clone();
        curr_line = next_line.clone();
        if curr_line.is_empty() {
            return sum;
        }
        next_line = match iterator.next() {
            None => Vec::new(),
            Some(line) => line.chars().collect(),
        };
    }
}

fn has_line_special_signs(pos: usize, len: usize, line: Vec<char>) -> bool {
    if line.is_empty() {
        return false;
    }

    let right = if pos == len { 0 } else { pos - len - 1 };
    let left = if pos >= line.len() {
        line.len() - 1
    } else {
        pos
    };
    let slice = &line[right..=left];

    slice
        .iter()
        .filter(|c| !c.is_numeric())
        .filter(|c| **c != '.')
        .count()
        != 0
}

#[cfg(test)]
mod test {
    use crate::day_3::{calculate, has_line_special_signs};

    const TEXT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn calculate_gear() {
        let result = calculate("...5..\n..*...\n......".lines());
        assert_eq!(result, 5);

        let result = calculate(TEXT.lines());
        assert_eq!(result, 4361);
    }

    #[test]
    fn test_check_line() {
        let expected = vec![
            (3, 2, vec!['.', '.', '.', '.', '.'], false),
            (3, 2, vec!['1', '1', '1', '1', '1'], false),
            (3, 2, vec!['.', '.', '*', '.', '.'], true),
            (3, 2, vec!['.', '1', '1', 'a', '.'], true),
            (3, 2, vec!['.', '.', '.', '9', '.'], false),
            (3, 2, vec!['.', '.', '.', '9', 'a'], false),
            (3, 2, vec!['*', '.', '.', '9', 'a'], true),
            (3, 2, vec!['*', '.', '.', '9', 'a'], true),
            (4, 2, vec!['*', '.', '5', '9', 'a'], true),
            (4, 2, vec!['*', '.', '5', '9', '.'], false),
            (3, 1, vec!['*', '.', '.', '9', 'a'], false),
            (6, 2, vec!['*', '.', '.', '9', 'a', '.', 'a'], true),
            (7, 2, vec!['*', '.', '.', '9', 'a', '.', 'a'], true),
            (2, 2, vec!['*', '.', '.', '9', 'a'], true),
            (
                6,
                3,
                vec!['*', '.', '.', '9', 'a', '*', '.', '.', '9', 'a'],
                true,
            ),
        ];

        for ex in expected {
            assert_eq!(has_line_special_signs(ex.0, ex.1, ex.2), ex.3);
        }
    }
}
