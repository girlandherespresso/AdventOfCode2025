const DIAL_SIZE: i32 = 100;

pub fn solve(input: &str, part: u8) -> u32 {
    let dial_position: i32 = 50;
    let rotations = parse_input(input);
    
    calc_zeros(dial_position, &rotations, part)
}

fn parse_input(input: &str) -> Vec<i32> {
    let lines = input.trim().lines();
    let mut rotations: Vec<i32> = Vec::new();
    
    for line in lines {
        let rotation_dir;

        if line.starts_with("R") {
            rotation_dir = 1;
        } else if line.starts_with("L") {
            rotation_dir = -1;
        } else {
            panic!("Invalid rotation direction!");
        }
        
        // Okay to use .expect() here because program should crash if parsing fails (invalid input)
        let rotation_amount: i32 = line[1..]
            .parse()
            .expect("Failed to parse string to integer");

        let rotation: i32 = rotation_dir * rotation_amount;
        rotations.push(rotation);
    }

    rotations
}


fn calc_zeros(dial_pos: i32, rotations: &Vec<i32>, part: u8) -> u32 {
    let mut cur_dial_pos = dial_pos;
    let mut zeros_passed: u32 = 0;

    for rotation in rotations.iter() {
        let mut next_dial_pos = cur_dial_pos + rotation;
        if part == 1 {
            next_dial_pos = next_dial_pos.rem_euclid(DIAL_SIZE);
        }
       
        match next_dial_pos {
            0 => {
                zeros_passed += 1;
            },
            1..DIAL_SIZE => {}, // Going from 1-99 -> 1-99 never passes 0
            num if num >= DIAL_SIZE => { // When part==1, num < 100 so match never occurs
                zeros_passed += (num / DIAL_SIZE) as u32;
            },
            _ => { // when part==1, next_dial_pos >= 0 so match never occurs
                if cur_dial_pos != 0 {
                    zeros_passed += 1;
                }
                zeros_passed += (next_dial_pos / DIAL_SIZE).abs() as u32;
            }
        }
        next_dial_pos = next_dial_pos.rem_euclid(DIAL_SIZE);

        cur_dial_pos = next_dial_pos;
    }

    zeros_passed
}

