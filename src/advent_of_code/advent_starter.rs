mod day_1;
mod day_2;
mod day_3;

use crate::day_2::calculate_games;
use crate::day_3::calculate_gear;
use day_1::calculate_calibration_document;

fn main() {
    println!("Hello advent!");
    println!("*************");
    println!("Day 1");
    let answer_day_1 = calculate_calibration_document("./src/advent_of_code/data/data_1.txt");
    println!("Day 1 answer: {}", answer_day_1);
    println!("*************\n");
    println!("*************");
    println!("Day 2");
    let answer_day_2 = calculate_games("./src/advent_of_code/data/data_2.txt");
    println!("Day 2 answer: {}", answer_day_2);
    println!("*************\n");
    println!("*************");
    println!("Day 3");
    let answer_day_3 = calculate_gear("./src/advent_of_code/data/data_3.txt");
    println!("Day 3 answer: {}", answer_day_3);
    println!("*************\n");
}
