const DIAL_SIZE: i32 = 100;

pub fn solve(input: &str) -> u32 {
    let dial_position: i32 = 50;

    let rotations = parse_input(input);
    let dial_positions = calc_dial_positions(dial_position, rotations);
    let total_zeros = calc_total_zeros(dial_positions);

    total_zeros
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
            panic!("Invalid rotation!");
        }

        let rotation_amount: i32 = line[1..]
            .parse()
            .expect("Failed to parse string to integer");

        let rotation: i32 = rotation_dir * rotation_amount;
        rotations.push(rotation);
    }

    rotations
}

fn calc_total_zeros(positions: Vec<i32>) -> u32 {
    let pos_iter = positions.iter();
    let mut zero_count: u32 = 0;
    
    for pos in pos_iter {
        if *pos == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

fn calc_dial_positions(dial_pos: i32, rotations: Vec<i32>) -> Vec<i32> {
    let rot_iter = rotations.iter();
    let mut cur_dial_pos = dial_pos;
    
    let mut positions = Vec::new();

    for rot in rot_iter {
        cur_dial_pos = rotate_dial(cur_dial_pos, *rot);
        positions.push(cur_dial_pos);
    }

    positions
}

fn rotate_dial(dial_pos: i32, rotation: i32) -> i32 {
    (dial_pos + rotation).rem_euclid(DIAL_SIZE)
}
