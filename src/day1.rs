const DIAL_SIZE: i32 = 100;

pub fn solve() {
    println!("Solving day 1...");

    let mut dial_position: i32 = 50;

    let rotations = vec![50,-10,20,99,-10,1];

    let dial_positions = calc_dial_positions(dial_position, rotations);

    let dial_iter = dial_positions.iter();

    for dial_pos in dial_iter {
        println!("Dial Position: {dial_pos}");
    }

    let total_zeros = calc_total_zeros(dial_positions);

    println!("Total zeros: {total_zeros}");

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
