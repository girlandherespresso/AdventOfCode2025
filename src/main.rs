use std::fs;

//mod day1;
mod day2;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to Advent of Code!");
    
    /* Day 1
    let day1_input_file = "src/inputs/day1.txt";
    println!("--- Solution to Day 1 ---\nPart 1:\n{}\n Part 2:\n{}", 
        day1::solve(&read_file(day1_input_file), 1),
        day1::solve(&read_file(day1_input_file), 2));
    */

    /* Day 2 */
    let day2_input_file = "src/inputs/day2.txt";
    println!("--- Solution to Day 2 ---\n{}",
        day2::solve(&read_file(day2_input_file), 1));



    Ok(())
}


fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path)
        .expect("file could not be read")
}
