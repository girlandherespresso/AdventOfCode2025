use std::collections::HashMap;
use std::cmp;

struct Range {
    start: u64,
    end: u64,
}

pub fn solve(input: &str, part: u8) -> u64 {
    let ranges = parse_input(input); 
    let ids = calc_all_ids(&ranges);

    // Get only the invalid ids
    let mut invalid_ids: Vec<u64> = Vec::new();
    for id in ids.iter() {
        let invalid_id = get_invalid_id(*id, part);
        match invalid_id {
            Some(num) => invalid_ids.push(num),
            None => (),
        }
    }
    // Sum invalid ids
    let mut invalid_sum: u64 = 0;
    for invalid_id in invalid_ids.iter() {
        invalid_sum += invalid_id;
    }

    invalid_sum
}

fn get_invalid_id(id: u64, part: u8) -> Option<u64> {
    let id_string: String = id.to_string();
    let mut patterns: HashMap<&str, usize> = HashMap::new();

    // Get all patterns in the string
    // Only pattenrs in the first half matter; larger patterns would not repeat
    // Additionally, pattern must exist in entire string so must always include the first digit 
    for len in 1..=(id_string.len() / 2) {
        if id_string.len() % len == 0 {
            patterns.insert(
                &id_string[0..len],
                len,
            );
        }
    }
    
    //print!("{id}");

    // Check through patterns, eliminating failed patterns
    for (pattern, pattern_len) in &patterns {
        // Now check through the string each 'pattern_len' characters for equality to pattern
        let mut valid: bool = true;
        for index in 0..(id_string.len() / pattern_len) {
            if (part == 1) && index >= 2 {
                valid = false;
                break;
            }
            let start = index * pattern_len;
            let end = (index + 1) * pattern_len;
            let cur_slice = &id_string[start..(cmp::min(end, id_string.len()))]; // May fail out of range? 
            //println!("{pattern}");
            if cur_slice != *pattern {
                valid = false;
                break;
            }
            //println!("Start: {start} | End: {end} | Current pattern: {cur_slice} | Len: {} | Valid: {valid}", patterns.get(pattern).unwrap());
        }
        if valid {
            //println!(" valid: {valid}");
            return Some(id);
        }
    }
    
    //println!(" valid: false");

    None
}

fn calc_all_ids(ranges: &Vec<Range>) -> Vec<u64> {
    let mut ids: Vec<u64> = Vec::new();

    for range in ranges.iter() {
        ids.extend((range.start..=range.end).collect::<Vec<u64>>());
    }

    ids
}

fn parse_input(input: &str) -> Vec<Range> {
    let range_strings: Vec<&str> = input.trim().split(",").collect();
    let mut ranges: Vec<Range> = Vec::new();
    for range_string in range_strings {
        let nums: Vec<&str> = range_string.split("-").collect();
        let min: u64 = nums[0].parse::<u64>().unwrap();
        let max: u64 = nums[1].parse::<u64>().unwrap();
        let range = Range {
            start: min,
            end: max,
        };
        ranges.push(range);
    }
    
    ranges
}
