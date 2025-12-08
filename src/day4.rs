use std::cmp;

struct GridSpace {
    has_roll: bool,
    accessible: bool,
}

struct Grid {
    cols: usize,
    rows: usize,
    spaces: Vec<GridSpace>,
}

impl Grid {
    fn get_space(&mut self, col: usize, row: usize) -> &mut GridSpace {
        let index = (row * self.cols) + col;
        &mut self.spaces[index]
    }
}

pub fn solve(input: &str, part: u8) -> usize {
    let mut grid = parse_input(input); 
  
    for c in 0..grid.cols {
        for r in 0..grid.rows {
            is_roll_accessible(&mut grid, c, r, 4);
        }
    }
    
    let mut num_accessible_rolls = 0;
    let mut index = 0;
    for space in grid.spaces.iter() {
        if index % grid.cols == 0 {
            println!("");
        }
        let mark = match space {
            GridSpace { has_roll: true, accessible: true } => {
                num_accessible_rolls += 1;
                'x'
            },
            GridSpace { has_roll: true, accessible: false } => '@',
            GridSpace { has_roll: false, .. } => '.',
        };
        print!("{mark}");
        index += 1;
    }
    println!("");

    num_accessible_rolls
}

fn is_roll_accessible(grid: &mut Grid, column: usize, row: usize, max_surrounding_rolls: usize) -> bool {
    let min_col: usize = match column {
        0 => 0,
        _ => column - 1,
    };
    let max_col = cmp::min(column + 1, grid.cols - 1);
    let min_row: usize = match row {
        0 => 0,
        _ => row - 1,
    };
    let max_row = cmp::min(row + 1, grid.rows - 1);

    let mut spaces_to_check: Vec<(usize, usize)> = Vec::new();
    
    for c in min_col..=max_col {
        for r in min_row..=max_row {
            if c == column && r == row {
                continue;
            }
            spaces_to_check.push((c, r));
        }
    }

    //println!("Col: {column}, Row: {row}");

    //println!("Num Spaces to Check: {}", spaces_to_check.len());

    
    let mut adjacent_roll_count = 0;
    for space in spaces_to_check.iter() {
        if grid.get_space(space.0, space.1).has_roll {
            adjacent_roll_count += 1;
        }
    }

    //println!("Adjacent Rolls: {adjacent_roll_count}");

    let accessible = adjacent_roll_count < max_surrounding_rolls;
    grid.get_space(column, row).accessible = accessible;
    accessible
}


fn parse_input(input: &str) -> Grid {
    let input_rows = input.lines();
    
    let mut cols = 0;
    let mut rows = 0;
    let mut spaces: Vec<GridSpace> = Vec::new();
    
    for input_row in input_rows {
        for (index, c)  in input_row.char_indices() {
            if index >= cols {
                cols = index + 1;
            }
            let has_roll: bool = match c {
                '@' => true,
                _ => false,
            };
            let grid_space = GridSpace {
                has_roll: has_roll,
                accessible: true, // default to true and assign to false when checked
            };
            spaces.push(grid_space);
            print!("{c}");
        }
        rows += 1;
        println!("");
    }

    Grid{
        cols: cols,
        rows: rows,
        spaces: spaces,
    }
}

