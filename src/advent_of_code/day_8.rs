use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;
use std::str::Lines;

pub fn calculate_map(path: &str) -> i64 {
    let lines = read_to_string(Path::new(path)).unwrap();
    let lines = lines.lines();
    calculate(lines)
}

fn calculate(lines: Lines) -> i64 {
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
    let mut currs = vec![];

    for key in map.keys().into_iter() {
        if key.ends_with('A') {
            currs.push(key.clone());
        }
    }

    loop {
        for c in path.chars().into_iter() {
            counter += 1;
            let mut new_currs = vec![];
            for curr in currs {
                let node = map.get(curr.as_str()).unwrap();
                new_currs.push(if c == 'L' {
                    node.0.clone()
                } else {
                    node.1.clone()
                });
            }
            if are_all_nodes_ends_with_z(&new_currs) {
                return counter;
            }
            if counter % 100_000_000 == 0 {
                println!("wow {}", counter);
            }
            currs = new_currs.clone();
        }
    }
}

fn are_all_nodes_ends_with_z(nodes: &Vec<String>) -> bool {
    for node in nodes.into_iter() {
        let a = node.chars().collect::<Vec<_>>();
        if a[2] != 'Z' {
            return false;
        }
    }

    return true;
}

#[cfg(test)]
mod test {
    use crate::day_8::calculate;

    const TEXT: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const TEXT2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_calculate() {
        assert_eq!(calculate(TEXT2.lines()), 6);
    }
}
