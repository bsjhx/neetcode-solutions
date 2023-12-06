pub fn calculate_races() -> i64 {
    let input = vec![(35937366, 212206012011044)];
    calculate(input)
}

fn calculate(races: Vec<(i64, i64)>) -> i64 {
    let mut sum = 1;

    for race in races {
        let (race_time, record) = race;

        // let mut c = HashSet::new();
        let mut c = 0;
        for i in 1..=race_time {
            let distance = i * (race_time - i);

            if distance > record {
                c += 1;
            }
        }
        println!("Result {}", c);
        sum *= c;
    }

    sum
}

#[cfg(test)]
mod test {
    use crate::day_6::calculate;

    #[test]
    fn test() {
        assert_eq!(calculate(vec![(7, 9), (15, 40), (30, 200)]), 288);
    }
}
