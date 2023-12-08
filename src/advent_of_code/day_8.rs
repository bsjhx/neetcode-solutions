use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;
use std::str::Lines;

pub fn calculate_map(path: &str) -> i32 {
    let lines = read_to_string(Path::new(path)).unwrap();
    let lines = lines.lines();
    calculate(lines)
}

fn calculate(lines: Lines) -> i32 {
    let lines = lines.into_iter().collect::<Vec<_>>();
    let mut path = "".to_string();
    let mut map = HashMap::new(); // str -> (str, str) N -> ( L,R)

    for (i, line) in lines.into_iter().enumerate() {
        if i == 0 {
            path = line.to_string();
        }
        if i > 1 {
            let cl = line.replace(" ", "");
            let cl1 = cl.split("=").map(|s| s.to_string()).collect::<Vec<_>>();
            let pp = cl1[1]
                .clone()
                .replace("(", "")
                .replace(")", "")
                .split(",")
                .map(|s| s.to_string())
                .collect::<Vec<_>>();
            map.insert(cl1[0].clone(), (pp[0].clone(), pp[1].clone()));
        }
    }

    let mut counter = 0;
    let mut curr = "AAA".to_string();
    loop {
        for c in path.chars().into_iter() {
            let node = map.get(curr.as_str()).unwrap();
            curr = if c == 'L' {
                node.0.clone()
            } else {
                node.1.clone()
            };

            counter += 1;
            if curr == "ZZZ" {
                return counter;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::day_8::calculate;

    const TEXT: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const TEXT2: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_calculate() {
        assert_eq!(calculate(TEXT.lines()), 6);
        assert_eq!(calculate(TEXT2.lines()), 2);
    }
}
