mod day_1;

use day_1::calculate_calibration_document;

fn main() {
    println!("Hello advent!");
    println!("*************");
    println!("Day 1");
    let answer_day_1 = calculate_calibration_document("./src/advent_of_code/data_1.txt");
    println!("Day 1 answer: {}", answer_day_1);
    println!("*************");
}
