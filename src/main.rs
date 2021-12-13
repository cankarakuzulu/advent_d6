use std::time::Instant;
use crate::file_reader::{convert_lines_to_strings, read_lines};
use crate::line_parser::get_initial_school;

mod lanternfish_school;
mod file_reader;
mod line_parser;
mod tests;

fn main() {
    const INPUT: &str = "Data/input.txt";
    let mut ocean = get_initial_school(convert_lines_to_strings(read_lines(INPUT)));
    while ocean.days_passed != 256{
        let now = Instant::now();
        ocean.spend_one_day();
        let elapsed = now.elapsed().as_secs();
        println!("Day: {} elapsed: {} count:{}", ocean.days_passed, elapsed, ocean.count());

    }
    println!("There are {} lantern fishes in the ocean.", ocean.count());
}
