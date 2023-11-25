use std::collections::{HashMap, HashSet};

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut set: HashSet<i32> = HashSet::new();

    for num in nums {
        if set.contains(&num) {
            return true;
        } else {
            set.insert(num);
        }
    }

    false
}

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut map: HashMap<char, i32> = HashMap::new();
    let mut map2: HashMap<char, i32> = HashMap::new();

    for (i, c) in s.chars().enumerate() {
        let ct = t.chars().nth(i).unwrap();
        *map.entry(c.to_owned()).or_default() += 1;
        *map2.entry(ct.to_owned()).or_default() += 1;
    }

    map.iter().all(|(c, i)| {
        let a = map2.get(c).unwrap_or(&0);
        i == a
    })
}

pub fn is_anagram_better(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut hm = HashMap::<char, i32>::new();

    for (a, b) in s.chars().zip(t.chars()) {
        *hm.entry(a).or_default() += 1;
        *hm.entry(b).or_default() -= 1;
    }

    hm.into_values().all(|cnt| cnt == 0)
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for (e, n) in (&nums).iter().enumerate() {
        match map.get(&n) {
            None => {
                map.insert(target - n, e as i32);
            }
            Some(v) => {
                return vec![*v, e as i32];
            }
        }
    }

    vec![]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_contains_duplicate() {
        assert_eq!(contains_duplicate(vec![1, 2, 3, 1]), true);
        assert_eq!(contains_duplicate(vec![1, 3, 4]), false);
    }

    #[test]
    fn test_is_anagram() {
        assert_eq!(is_anagram("text".into(), "longer text".into()), false);
        assert_eq!(is_anagram("lol".into(), "oll".into()), true);
        assert_eq!(is_anagram("lol".into(), "olo".into()), false);
        assert_eq!(is_anagram("words".into(), "sword".into()), true);
        assert_eq!(is_anagram("one".into(), "two".into()), false);
        assert_eq!(is_anagram("aacc".into(), "ccac".into()), false);
    }

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![1, 5, 7, 9], 6), [0, 1]);
        assert_eq!(two_sum(vec![-1, -2, -3, -4, -5], -8), [2, 4]);
    }
}
