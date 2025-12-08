use std::fs;

//mod day1;
//mod day2;
//mod day3;
mod day4;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to Advent of Code!");
    
    /* Day 1
    let day1_input_file = "src/inputs/day1.txt";
    println!("--- Solution to Day 1 ---\nPart 1:\n{}\n Part 2:\n{}", 
        day1::solve(&read_file(day1_input_file), 1),
        day1::solve(&read_file(day1_input_file), 2));
    */

    /* Day 2 
    let day2_input_file = "src/inputs/day2.txt";
    println!("--- Solution to Day 2 ---\nPart 1:\n{}\n Part 2:\n{}",
        day2::solve(&read_file(day2_input_file), 1),
        day2::solve(&read_file(day2_input_file), 2));
    */

    /* Day 3
    let day3_input_file = "src/inputs/day3.txt"; 
    println!("--- Solution to Day 3 ---\nPart 1:\n{}\n Part 2:\n{}",
        day3::solve(&read_file(day3_input_file), 1),
        day3::solve(&read_file(day3_input_file), 2));*/

    /* Day 4 */
    let day4_input_file = "src/inputs/day4.txt";
    println!("--- Solution to Day 4 ---\nPart 1:\n{}\n Part 2:\n{}",
        day4::solve(&read_file(day4_input_file), 1),
        day4::solve(&read_file(day4_input_file), 2));
    //println!("{}",day4::solve(&read_file(day4_input_file), 2));

    Ok(())
}


fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path)
        .expect("file could not be read")
}
