use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;
use std::str::Lines;

pub fn calculate_poker(path: &str) -> i32 {
    let lines = read_to_string(Path::new(path)).unwrap();
    let lines = lines.lines();
    calculate(lines)
}

fn calculate(lines: Lines) -> i32 {
    let a = lines.collect::<Vec<_>>();
    let mut a = a
        .iter()
        .map(|s| s.split(" ").collect::<Vec<&str>>())
        .map(|s| (s[0], s[1]))
        .collect::<Vec<(&str, &str)>>();

    a.sort_by(|a, b| compare(a.0, b.0));
    println!("dfs");
    let mut sum = 0;
    for (i, c) in a.iter().enumerate() {
        sum += (i + 1) as i32 * c.1.parse::<i32>().unwrap();
    }

    sum
}

fn compare(h1: &str, h2: &str) -> Ordering {
    if h1 == h2 {
        return Ordering::Equal;
    }
    let rank1 = calculate_rank(h1);
    let rank2 = calculate_rank(h2);
    let mut cards = HashMap::new();
    cards.insert('A', 14);
    cards.insert('K', 13);
    cards.insert('Q', 12);
    cards.insert('T', 10);
    cards.insert('9', 9);
    cards.insert('8', 8);
    cards.insert('7', 7);
    cards.insert('6', 6);
    cards.insert('5', 5);
    cards.insert('4', 4);
    cards.insert('3', 3);
    cards.insert('2', 2);
    cards.insert('J', 1);

    if rank1 == rank2 {
        let zipped = h1.chars().into_iter().zip(h2.chars().into_iter());
        for (c1, c2) in zipped {
            let a = cards.get(&c1).unwrap().cmp(cards.get(&c2).unwrap());
            match a {
                Ordering::Less => return Ordering::Less,
                Ordering::Equal => (),
                Ordering::Greater => return Ordering::Greater,
            }
        }
    }

    if rank1 > rank2 {
        return Ordering::Greater;
    }

    Ordering::Less
}

fn calculate_rank(hand: &str) -> i32 {
    let mut m = HashMap::new();
    for c in hand.chars().into_iter() {
        *m.entry(c).or_default() += 1;
    }

    // revalidate Js
    if m.get(&'J').is_some() {
        let js = *m.get(&'J').unwrap();
        m.remove(&'J');
        let mut max = 0;
        let mut max_key = 'a';
        for el in m.iter() {
            if el.1 > &max {
                max_key = *el.0;
                max = *el.1;
            }
        }
        *m.entry(max_key).or_default() += js;
    }

    let values = m.values().collect::<Vec<_>>();
    // 12345
    if values.len() == 5 {
        return 1;
    }

    // 111111
    if values.len() == 1 {
        return 7;
    }

    // 11115
    if values.contains(&&4) {
        return 6;
    }

    if values.contains(&&3) {
        // 11122
        if values.contains(&&2) {
            return 5;
        } else {
            // 11123
            return 4;
        }
    } else if values.contains(&&2) {
        // 11223
        if values.len() == 3 {
            return 3;
        } else {
            // 11234
            return 2;
        }
    }

    1
}

#[cfg(test)]
mod test {
    use crate::day_7::*;

    #[test]
    fn test_all() {
        let t = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        assert_eq!(calculate(t.lines()), 5905);
    }

    #[test]
    fn test_compare() {
        assert_eq!(compare("AAAAA", "AAQAA"), Ordering::Greater);
        assert_eq!(compare("KAAAA", "AAKAA"), Ordering::Less);
        assert_eq!(compare("23456", "34562"), Ordering::Less);
    }

    #[test]
    fn test_rank() {
        assert_eq!(calculate_rank("AAAAA"), 7);
        assert_eq!(calculate_rank("AAA5A"), 6);
        assert_eq!(calculate_rank("15841"), 2);
        assert_eq!(calculate_rank("A2A3A"), 4);
        assert_eq!(calculate_rank("45545"), 5);
        assert_eq!(calculate_rank("12345"), 1);
        assert_eq!(calculate_rank("AKA99"), 3);
        assert_eq!(calculate_rank("T55J5"), 6);
        assert_eq!(calculate_rank("23J56"), 2);
    }
}
