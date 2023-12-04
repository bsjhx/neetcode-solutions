use std::fs::read_to_string;
use std::path::Path;
use std::str::Lines;

pub fn calculate_games(path: &str) -> i32 {
    let lines = read_to_string(Path::new(path)).unwrap();
    let lines = lines.lines();
    calculate(lines)
}

fn calculate(lines: Lines) -> i32 {
    let mut sum = 0;

    for line in lines.into_iter() {
        let game = line.split(":").collect::<Vec<&str>>()[1];
        let sets = game.split(";");

        let (mut r, mut g, mut b) = (0, 0, 0);

        for set in sets.into_iter() {
            let single_reveal = set.split(",");

            for revealed_cubes in single_reveal.into_iter() {
                let temp = revealed_cubes.split(" ").collect::<Vec<&str>>();
                match temp[2] {
                    "red" => {
                        if get_count(&temp) > r {
                            r = get_count(&temp)
                        }
                    }
                    "green" => {
                        if get_count(&temp) > g {
                            g = get_count(&temp)
                        }
                    }
                    "blue" => {
                        if get_count(&temp) > b {
                            b = get_count(&temp)
                        }
                    }
                    &_ => {}
                }
            }
        }
        sum = sum + (r * g * b);
    }
    sum
}

fn get_count(temp: &Vec<&str>) -> i32 {
    temp[1].parse::<i32>().unwrap()
}

#[cfg(test)]
mod test {
    use crate::day_2::calculate;

    const TEXT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_calculate_games() {
        assert_eq!(calculate(TEXT.lines()), 2286);
    }
}
