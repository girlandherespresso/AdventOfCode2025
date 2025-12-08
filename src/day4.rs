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
    let max_iterations = match part {
        1 => 1,
        _ => usize::MAX,
    };
    remove_accessible_rolls(&mut grid, 4, max_iterations)  
}

fn remove_accessible_rolls(grid: &mut Grid, surrounding_roll_limit: usize, max_iterations: usize) -> usize {
    let mut iteration = 0;
    let mut removed_rolls = 0;
    let mut last_removed_rolls = 1;
    while last_removed_rolls != removed_rolls && iteration < max_iterations {
        for c in 0..grid.cols {
            for r in 0..grid.rows {
                check_roll_accessible(grid, c, r, surrounding_roll_limit);
            }
        }

        last_removed_rolls = removed_rolls;
        for space in grid.spaces.iter_mut() {
            match space {
                GridSpace { has_roll: true, accessible: true } => {
                    removed_rolls += 1;
                    space.has_roll = false;
                }
                _ => {},
            }
        }
        //print_grid(grid, removed_rolls);
        iteration += 1;
    }

    removed_rolls
}

fn print_grid(grid: &mut Grid, removed_rolls: usize) {
    let mut index = 0;
    for space in grid.spaces.iter_mut() {
        if index % grid.cols == 0 {
            println!("");
        }
        let mark = match space {
            GridSpace { has_roll: true, accessible: true } => 'x',
            GridSpace { has_roll: true, accessible: false } => '@',
            GridSpace { has_roll: false, .. } => '.',
        };
        print!("{mark}");
        index += 1;
    }
    println!("\n{removed_rolls}");

}

fn check_roll_accessible(grid: &mut Grid, column: usize, row: usize, surrounding_roll_limit: usize) -> bool {
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

    let mut adjacent_roll_count = 0;
    for space in spaces_to_check.iter() {
        if grid.get_space(space.0, space.1).has_roll {
            adjacent_roll_count += 1;
        }
    }

    let accessible = adjacent_roll_count < surrounding_roll_limit;
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
                accessible: true,
            };
            spaces.push(grid_space);
        }
        rows += 1;
    }

    Grid{
        cols: cols,
        rows: rows,
        spaces: spaces,
    }
}

