use aoc_2020::utils::read_file;
use std::cell::RefCell;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let final_map = process_map();
    let occupied_seats = final_map.iter()
        .flatten().into_iter()
        .filter(|&&c| c == MapCell::Occupied).count();
    println!("There are {} occupied seats", occupied_seats);
    println!("{}", now.elapsed().as_millis());
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum MapCell {
    Occupied,
    Empty,
    Floor,
}

impl MapCell {
    fn from_char(c: &char) -> Self {
        match c {
            'L' => MapCell::Empty,
            '#' => MapCell::Occupied,
            '.' => MapCell::Floor,
            _ => panic!("Invalid character {}", c)
        }
    }
}

fn process_map() -> Vec<Vec<MapCell>> {
    let map: Vec<Vec<MapCell>> = read_file("resources/problem_11_input.txt").iter()
        .map(|row| row.chars().map(|c| MapCell::from_char(&c)).collect::<Vec<MapCell>>())
        .collect();

    let row_count = map.len();
    let column_count = map[0].len();
    // Keep two copies of the map. We will process one at a time and store results in the other
    let map_to_process: RefCell<Vec<Vec<MapCell>>> = RefCell::new(map.clone());
    let map_to_update: RefCell<Vec<Vec<MapCell>>> = RefCell::new(map.clone());

    let mut map_changed = true;
    while map_changed {
        // Process changes to map
        // let map_to_process = &maps[map_processing_idx];
        for y in 0..row_count {
            let map_to_process = map_to_process.borrow();
            let row_above = if y > 0 { map_to_process.get(y - 1) } else { None };
            let row_below = map_to_process.get(y + 1);
            let row = &map_to_process[y];

            for x in 0..column_count {
                let mut adjacents: Vec<MapCell> = Vec::with_capacity(8);

                // Get adjacent cells from row above
                get_adjacent_from_row(&mut adjacents, x, &row_above);

                // Get adjacent cells to left and right
                if x > 0 {
                    if let Some(left) = row.get(x - 1) {
                        adjacents.push(left.clone());
                    }
                }
                if let Some(right) = row.get(x + 1) {
                    adjacents.push(right.clone());
                }

                // Get adjacent cells from row below
                get_adjacent_from_row(&mut adjacents, x, &row_below);

                // Now that we have adjacents, determine if current cell should change
                // Count the cell types
                let occupied_count = adjacents.iter()
                    .filter(|&&a| a == MapCell::Occupied).count();

                map_to_update.borrow_mut()[y][x] = match map_to_process[y][x] {
                    MapCell::Occupied => {
                        if occupied_count >= 4 {
                            MapCell::Empty
                        } else {
                            MapCell::Occupied
                        }
                    }
                    MapCell::Empty => {
                        if occupied_count == 0 {
                            MapCell::Occupied
                        } else {
                            MapCell::Empty
                        }
                    }
                    MapCell::Floor => MapCell::Floor
                }
            }
        }

        // Determine if changes happened
        map_changed = false;
        for y in 0..row_count {
            for x in 0..column_count {
                let process_cell = map_to_process.borrow()[y][x];
                let update_cell = map_to_update.borrow()[y][x];
                if process_cell != update_cell {
                    map_changed = true;
                }
            }
        }
        map_to_process.swap(&map_to_update);
    }
    let map_to_process = map_to_process.borrow().clone();
    map_to_process
}

fn get_adjacent_from_row(adjacents: &mut Vec<MapCell>, x: usize, row: &Option<&Vec<MapCell>>) {
    if let Some(row) = row {
        if x > 0 {
            if let Some(left_column) = row.get(x - 1) {
                adjacents.push(left_column.clone());
            }
        }
        if let Some(same_column) = row.get(x) {
            adjacents.push(same_column.clone());
        }
        if let Some(above_right) = row.get(x + 1) {
            adjacents.push(above_right.clone());
        }
    }
}
