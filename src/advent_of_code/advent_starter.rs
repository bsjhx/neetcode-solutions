mod day_1;
mod day_2;

use crate::day_2::calculate_games;
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
    println!("*************");
}
