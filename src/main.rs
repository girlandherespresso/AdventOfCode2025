use std::fs;

mod day1;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to Advent of Code!");

    let day1_input_file = "src/inputs/day1.txt";
    println!("Solution to day 1:\n{}", day1::solve(&read_file(day1_input_file)));

    Ok(())
}


fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path)
        .expect("file could not be read")
}
