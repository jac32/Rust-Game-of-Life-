extern crate rand;

use std::cmp;
use rand::Rng;

const RAND_RATIO: u8 = 200;
const GRID_SIZE: usize = 100;

fn main() {

    let init_state = random_state();
    let mut state = init_state;
    let mut i = 0;
    while !all_dead(state) && i < 10000 {
        print_state(state);
        state = step(state);
        i = i+1;
    }
}

fn all_dead (state: [[bool; GRID_SIZE]; GRID_SIZE]) -> bool {
    for row in state.iter() {
        for cell in row.iter() {
            if *cell { return false }
        }
    }
    true
}

fn step(state: [[bool; GRID_SIZE]; GRID_SIZE]) -> [[bool; GRID_SIZE]; GRID_SIZE] {
    let mut new_state = [[false; GRID_SIZE]; GRID_SIZE];
    for (r, row) in state.iter().enumerate() {
        for (c, _) in row.iter().enumerate() {
            new_state[r][c] = step_cell(state, r, c);
        }
    }
    return new_state;
}

fn step_cell(state: [[bool; GRID_SIZE]; GRID_SIZE], row_num: usize, col_num: usize) -> bool {
    let adj_count = count_adj(state, row_num, col_num);
    let alive = state[row_num][col_num];
    match adj_count {
        3 => true,
        2 => alive,
        1 => false,
        0 => false,
        _ => false,
    }
}

fn count_adj(state: [[bool; GRID_SIZE]; GRID_SIZE], row_num: usize, col_num: usize) -> u32 {

    let size = state.len() - 1;
    let row_start = if row_num == 0 { 0 } else { cmp::max(row_num - 1, 0) };
    let col_start = if col_num == 0 { 0 } else { cmp::max(col_num - 1, 0) };
    let row_finish = if row_num == size { size } else { cmp::min(row_num + 1, state.len() - 1) };
    let col_finish = if col_num == size { size } else { cmp::min(col_num + 1, state.len() - 1) };

    let mut count = 0;

    for r in row_start .. row_finish + 1 {
        for c in col_start .. col_finish +1 {
            if state[r][c] && (r != row_num || c != col_num) {
                count = count + 1;
            }
        }
    }
   count
}


fn random_state() -> [[bool; GRID_SIZE]; GRID_SIZE] {

    let mut new_state = [[false; GRID_SIZE]; GRID_SIZE];

    let mut rng = rand::thread_rng();
    let mut rand_grid = [0u8; GRID_SIZE*GRID_SIZE];
    rng.fill_bytes(&mut rand_grid);

    for (i, val) in rand_grid.iter().enumerate() {
        let cell = *val > RAND_RATIO;
        new_state[i/GRID_SIZE][i%GRID_SIZE] = cell;
    }

    return new_state;
}


fn print_state(state: [[bool; GRID_SIZE]; GRID_SIZE]) {

    csi();
    print!("2J");
    for row in state.iter() {
        for cell in row.iter() {
            print_cell(*cell);
        }
        print!("\n");
    }
    print!("\n");
}

fn csi() {
    print!("{}[", '\x1B');
}

fn print_cell(alive: bool) {
    if alive {
        print!("X");
    } else {
        print!("-");
    }
}
