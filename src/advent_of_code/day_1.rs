use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;
use std::str::Lines;
use std::string::ToString;

pub fn calculate_calibration_document(path: &str) -> i32 {
    let lines = read_to_string(Path::new(path)).unwrap();
    let lines = lines.lines();
    calculate(lines)
}

fn calculate(lines: Lines) -> i32 {
    let mut sum = 0;
    let mut c = 0;

    for line in lines.into_iter() {
        sum = sum + find_first_and_last_numbers(line);
        c = c + 1;
    }

    println!("TOTAL: {}", c);

    sum
}

fn replace_words_number_with_digits(s: &str) -> String {
    let words = create_words_map();
    let mut s2 = s.to_string();
    let (mut min, mut max) = (i32::MAX, -1);
    let (mut min_word, mut max_word) = ("", "");
    for word in words.keys().into_iter() {
        let found = s2.find(word);
        match found {
            None => {}
            Some(i) => {
                let i = i as i32;
                if i < min {
                    min = i;
                    min_word = word;
                }
            }
        }
        let found = s2.rfind(word);
        match found {
            None => {}
            Some(i) => {
                let i = i as i32;
                if i > max {
                    max = i;
                    max_word = word;
                }
            }
        }
    }

    if !min_word.is_empty() {
        // s2 = s2.replace(min_word, &*words.get(min_word).unwrap().to_string());
        s2.insert(
            min as usize,
            words.get(min_word).unwrap().to_string().parse().unwrap(),
        );
        max = max + 1;
    }

    if !max_word.is_empty() {
        // s2 = s2.replace(max_word, &*words.get(max_word).unwrap().to_string());
        s2.insert(
            max as usize,
            words.get(max_word).unwrap().to_string().parse().unwrap(),
        );
    }

    s2.to_string()
}

fn find_first_and_last_numbers(s: &str) -> i32 {
    if s.is_empty() {
        return 0;
    }

    print!("{} - ", s);

    let s = replace_words_number_with_digits(s);

    let s: Vec<char> = s
        .to_lowercase()
        .chars()
        .filter(|c| ('0'..='9').contains(c))
        .collect();

    print!("{:?} - ", s);

    let mut result = s[0].to_string();
    result.push(s[s.len() - 1]);

    println!("{}", result);

    result.parse::<i32>().unwrap()
}

fn create_words_map() -> HashMap<String, i32> {
    let mut words: HashMap<String, i32> = HashMap::new();
    for (i, w) in vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .iter()
    .enumerate()
    {
        words.insert(w.to_string(), (i + 1) as i32);
    }

    words
}

#[cfg(test)]
mod test {
    use crate::day_1::{calculate, replace_words_number_with_digits};

    #[test]
    fn test_calc() {
        let lines = "1ala5\ngfh5gf8hfg6fg".lines();
        assert_eq!(calculate(lines), 15 + 56);

        let lines = "1alninea5\ngfh5gf8hfg6fg".lines();
        assert_eq!(calculate(lines), 15 + 56);

        let lines = "onealninea5\ngfh5gf8hfg6fg".lines();
        assert_eq!(calculate(lines), 15 + 56);

        let lines = "dsfsd9fsdfsd\n1ala5\ngfh5gfhfg6fg\n1".lines();
        assert_eq!(calculate(lines), 99 + 15 + 56 + 11);

        let lines = "3two7vrv2rlqc4nine\n1ala5\ngfh5gfhfg6fg\n1".lines();
        assert_eq!(calculate(lines), 39 + 15 + 56 + 11);

        let lines = "3two7vrv2rlqc4nine\n1ala5\ngfh5gfhfg6fg\n1".lines();
        assert_eq!(calculate(lines), 39 + 15 + 56 + 11);

        let lines = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen".lines();
        assert_eq!(calculate(lines), 281);

        let lines =
            "threethreetcghqbv3rbnsdsk4sevennine\nfivetjmg5sevenjp\n121\none52ctrkkc\none\n8ninejseven5\noneninejseven5one".lines();
        assert_eq!(calculate(lines), 39 + 57 + 11 + 12 + 11 + 85 + 11);

        let lines =
            "gbddhhhhkgjltwothree57\n3six191dbxtkm\nseven3six\nsixsixsix\n5919xj\npierdone\npierdoneone".lines();
        assert_eq!(calculate(lines), 27 + 31 + 76 + 66 + 59 + 11 + 11);
    }
}
