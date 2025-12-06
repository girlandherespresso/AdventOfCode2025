struct MaxDigit {
    max_digit: usize,
    index: usize,
}

pub fn solve(input: &str, part: u8) -> usize {
    let banks = parse_input(input); 
    let mut joltage_total: usize = 0;

    for bank in banks.iter() {
        if part == 1 {
            joltage_total += find_largest_joltage(bank, 2);
        } else if part == 2 {
            joltage_total += find_largest_joltage(bank, 12);
        }
    }

    joltage_total
}

/* Brainstorming solution to part 1:
 * def last digit as nth digit
 * Step 1:
 *      Find largest digit in 1st through (n-1)th as first digit in final
 * Step 2:
 *      Search from index of largest digit to nth digit for largest remaining digit
 * Combine digits into final joltage
 */

fn find_largest_joltage(bank: &str, battery_count: usize) -> usize {
    let starting_end_index = bank.len() - battery_count;
    let mut start_index = 0;
    let mut max_digits: Vec<usize> = Vec::new();
    //println!("Bank: {bank}");
    
    // Iterate through each range to find the largest possible joltage in the bank slice
    for end_index in starting_end_index..(bank.len()) {
        let next_max_digit = find_largest_digit(bank, start_index, end_index);
        max_digits.push(next_max_digit.max_digit);
        //println!("Range: {start_index} - {end_index}");
        start_index = next_max_digit.index + 1;
    }

    // Add up the digits to create the final joltage

   // for digit in max_digits.iter() {
   //     print!("{digit}");
   // }
   // println!("");

    let mut digit_mult_factor = 1;
    let mut joltage = 0;
    for digit in max_digits.iter().rev() {
        joltage += digit * digit_mult_factor;
        digit_mult_factor *= 10;
    }


   // let first_max_digit = find_largest_digit(bank, 0, bank.len() - 1);
   // let starting_index = first_max_digit.index + 1;
   // let second_max_digit = find_largest_digit(bank, starting_index, bank.len()).max_digit;

   // let total = (10 * first_max_digit.max_digit) + second_max_digit;
    //println!("{total}");
    joltage
}

fn find_largest_digit(bank: &str, starting_index: usize, ending_index: usize) -> MaxDigit {
    let mut max_digit = 0;
    let mut max_index = 0;
    for index in starting_index..=ending_index {
        let index = index as usize;
        let cur_digit = bank[index..=index].parse().unwrap(); 
        if cur_digit > max_digit {
            max_digit = cur_digit;
            max_index = index;
        }
    }

    MaxDigit {
        max_digit: max_digit,
        index: max_index,
    }
}


fn parse_input(input: &str) -> Vec<String> {
    let mut banks: Vec<String> = Vec::new();
    
    for line in input.lines() {
        banks.push(String::from(line));
        //println!("{}",banks.last().unwrap());
    }

    banks
}
