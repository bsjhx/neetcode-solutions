use std::fs::read_to_string;
use std::path::Path;
use std::str::Lines;

// only 12 red cubes, 13 green cubes, and 14 blue cubes

const RED: i32 = 12;
const GREEN: i32 = 13;
const BLUE: i32 = 14;

pub fn calculate_games(path: &str) -> i32 {
    let lines = read_to_string(Path::new(path)).unwrap();
    let lines = lines.lines();
    calculate(lines)
}

// Game 1: 2 blue, 3 red; 3 green, 3 blue, 6 red; 4 blue, 6 red; 2 green, 2 blue, 9 red; 2 red, 4 blue
fn calculate(lines: Lines) -> i32 {
    let mut sum = 0;

    for (c, line) in lines.enumerate() {
        let mut games = line.split(":").collect::<Vec<&str>>()[1];
        let games = games.split(";");

        let mut is_possible = true;

        for game in games.into_iter() {
            let pos = game.split(",");

            for po in pos.into_iter() {
                let ber = po.split(" ").collect::<Vec<&str>>();
                match ber[2] {
                    "red" => {
                        if ber[1].parse::<i32>().unwrap() > RED {
                            is_possible = false;
                        }
                    }
                    "green" => {
                        if ber[1].parse::<i32>().unwrap() > GREEN {
                            is_possible = false;
                        }
                    }
                    "blue" => {
                        if ber[1].parse::<i32>().unwrap() > BLUE {
                            is_possible = false;
                        }
                    }
                    &_ => {}
                }
            }
        }
        if is_possible {
            sum += (c + 1) as i32;
        }
    }
    sum
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
        assert_eq!(calculate(TEXT.lines()), 8);
    }
}
