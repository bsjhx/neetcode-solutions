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
        for (i, c) in curr_line.clone().into_iter().enumerate() {
            if c == '*' {
                match find(i, &prev_line, &curr_line, &next_line) {
                    None => (),
                    Some(ratio) => {
                        sum += ratio;
                    }
                };
            }
        }

        prev_line = curr_line.clone();
        curr_line = next_line.clone();
        // println!("{:?}", curr_line);
        if curr_line.is_empty() {
            return sum;
        }
        next_line = match iterator.next() {
            None => Vec::new(),
            Some(line) => line.chars().collect(),
        };
    }
}

fn find(star_pos: usize, prev: &Vec<char>, curr: &Vec<char>, next: &Vec<char>) -> Option<i32> {
    let mut res = find_numbers_in(star_pos, prev);
    let mut res2 = find_numbers_in(star_pos, curr);
    let mut res3 = find_numbers_in(star_pos, next);
    res.append(&mut res2);
    res.append(&mut res3);

    if res.len() != 2 {
        None
    } else {
        Some(res[0] * res[1])
    }
}

fn find_numbers_in(star_pos: usize, prev: &Vec<char>) -> Vec<i32> {
    let mut temp = "".to_string();
    let mut res = vec![];
    for (i, c) in prev.iter().enumerate() {
        if c.is_numeric() {
            temp.push(*c);
        } else if !temp.is_empty() {
            let kkk = star_pos + temp.len() + 1;
            if i >= star_pos && i <= kkk {
                res.push(temp.parse::<i32>().unwrap());
                temp = "".to_string();
            } else {
                temp = "".to_string();
            }
        }
    }
    if temp != "" {
        res.push(temp.parse::<i32>().unwrap());
    }
    res
}

#[cfg(test)]
mod test {
    use crate::day_3::calculate;

    const TEXT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
.......755
...$..*...
.664.598..";

    #[test]
    fn calculate_gear() {
        let result = calculate("......\n.5*6..\n......".lines());
        assert_eq!(result, 30);

        let result = calculate(TEXT.lines());
        assert_eq!(result, 467835);
    }
}
