mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_6;
mod day_7;

use crate::day_2::calculate_games;
use crate::day_3::calculate_gear;
use crate::day_4::calculate_card;
use crate::day_6::calculate_races;
use crate::day_7::calculate_poker;
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
    println!("*************");
    println!("Day 4");
    let answer_day_4 = calculate_card("./src/advent_of_code/data/data_4.txt");
    println!("Day 4 answer: {}", answer_day_4);
    println!("*************\n");
    println!("Day 6");
    let answer_day_6 = calculate_races();
    println!("Day 6 answer: {}", answer_day_6);
    println!("*************\n");
    println!("*************");
    println!("Day 7");
    let answer_day_7 = calculate_poker("./src/advent_of_code/data/data_7.txt");
    println!("Day 7 answer: {}", answer_day_7);
    println!("*************\n");
}
