use aoc_2020::utils::read_file;
use std::cell::RefCell;
use std::time::Instant;

enum OccupiedTechnique {
    Adjacent,
    Visible,
}

fn main() {
    let now = Instant::now();
    // Part 1
    let final_map = process_map(OccupiedTechnique::Adjacent);
    let occupied_seats = final_map.iter().flatten().into_iter()
        .filter(|&&c| c == MapCell::Occupied).count();
    println!("Part 1. There are {} occupied seats", occupied_seats);
    println!("{}", now.elapsed().as_millis());

    // Part 2
    let final_map = process_map(OccupiedTechnique::Visible);
    let occupied_seats = final_map.iter().flatten().into_iter()
        .filter(|&&c| c == MapCell::Occupied).count();
    println!("Part 2. There are {} occupied seats", occupied_seats);
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

fn process_map(occupied_technique: OccupiedTechnique) -> Vec<Vec<MapCell>> {
    let map: Vec<Vec<MapCell>> = read_file("resources/problem_11_input.txt").iter()
        .map(|row| row.chars().map(|c| MapCell::from_char(&c)).collect::<Vec<MapCell>>())
        .collect();

    let row_count = map.len();
    let column_count = map[0].len();
    // Keep two copies of the map. We will process one at a time and store results in the other
    let map_to_process: RefCell<Vec<Vec<MapCell>>> = RefCell::new(map.clone());
    let map_to_update: RefCell<Vec<Vec<MapCell>>> = RefCell::new(map.clone());

    let empty_seat_count = match occupied_technique {
        OccupiedTechnique::Adjacent => 4,
        OccupiedTechnique::Visible => 5
    };

    let mut map_changed = true;

    while map_changed {
        map_changed = false;
        // Process changes to map
        // let map_to_process = &maps[map_processing_idx];
        for y in 0..row_count {
            let map_to_process = map_to_process.borrow();

            for x in 0..column_count {
                // Get number of occupied seats nearby based on what we're looking for
                let occupied_count = match occupied_technique {
                    // We are only looking for seats occupied adjacent to the current one
                    OccupiedTechnique::Adjacent => {
                        let row_above = if y > 0 { map_to_process.get(y - 1) } else { None };
                        let row_below = map_to_process.get(y + 1);
                        let row = &map_to_process[y];
                        let adjacents: Vec<MapCell> = get_adjacent_cells(x, &row_above, &row, &row_below);
                        // Now that we have adjacents, determine if current cell should change
                        adjacents.iter().filter(|&&a| a == MapCell::Occupied).count()
                    }
                    // We are looking for the first occupied seat in each direction
                    OccupiedTechnique::Visible => {
                        let mut occupied_count = 0;
                        // First get everything above the current cell
                        if y > 0 {
                            // Upper search
                            for offset in 1..map_to_process.len() {
                                if offset > y {
                                    break;
                                }
                                match map_to_process[y - offset][x] {
                                    MapCell::Occupied => {
                                        occupied_count += 1;
                                        break;
                                    }
                                    MapCell::Empty => {
                                        break;
                                    }
                                    MapCell::Floor => {}
                                }
                            }

                            // Up and left
                            for offset in 1..map_to_process.len() {
                                if offset > x || offset > y {
                                    break;
                                }
                                match map_to_process[y - offset][x - offset] {
                                    MapCell::Occupied => {
                                        occupied_count += 1;
                                        break;
                                    }
                                    MapCell::Empty => {
                                        break;
                                    }
                                    MapCell::Floor => {}
                                }
                            }

                            // Up and right
                            for offset in 1..map_to_process.len() {
                                if offset > y || x + offset >= map_to_process[y].len() {
                                    break;
                                }
                                match map_to_process[y - offset][x + offset] {
                                    MapCell::Occupied => {
                                        occupied_count += 1;
                                        break;
                                    }
                                    MapCell::Empty => {
                                        break;
                                    }
                                    MapCell::Floor => {}
                                }
                            }
                        }

                        // Now check everything to left and right
                        let mut left_found = false;
                        let mut right_found = false;
                        let row = &map_to_process[y];
                        let mut offset = 1;
                        while (offset <= x || offset < row.len()) && !(left_found && right_found) {
                            if !left_found && offset <= x {
                                if let Some(left) = row.get(x - offset) {
                                    process_map_cell_found(&left, &mut left_found, &mut occupied_count);
                                }
                            }
                            if !right_found {
                                if let Some(right) = row.get(x + offset) {
                                    process_map_cell_found(&right, &mut right_found, &mut occupied_count);
                                }
                            }
                            offset += 1;
                        }

                        // Now everything below!
                        for offset in 1..map_to_process.len() {
                            if y + offset >= map_to_process.len() {
                                break;
                            }
                            match map_to_process[y + offset][x] {
                                MapCell::Occupied => {
                                    occupied_count += 1;
                                    break;
                                }
                                MapCell::Empty => {
                                    break;
                                }
                                MapCell::Floor => {}
                            }
                        }

                        // Below and left
                        for offset in 1..map_to_process.len() {
                            if offset > x || y + offset >= map_to_process.len() {
                                break;
                            }
                            match map_to_process[y + offset][x - offset] {
                                MapCell::Occupied => {
                                    occupied_count += 1;
                                    break;
                                }
                                MapCell::Empty => {
                                    break;
                                }
                                MapCell::Floor => {}
                            }
                        }

                        // Below and right
                        for offset in 1..map_to_process.len() {
                            if x + offset >= row.len() || y + offset >= map_to_process.len() {
                                break;
                            }
                            match map_to_process[y + offset][x + offset] {
                                MapCell::Occupied => {
                                    occupied_count += 1;
                                    break;
                                }
                                MapCell::Empty => {
                                    break;
                                }
                                MapCell::Floor => {}
                            }
                        }
                        occupied_count
                    } // Visible check
                };

                map_to_update.borrow_mut()[y][x] = match map_to_process[y][x] {
                    MapCell::Occupied => {
                        if occupied_count >= empty_seat_count {
                            map_changed = true;
                            MapCell::Empty
                        } else {
                            MapCell::Occupied
                        }
                    }
                    MapCell::Empty => {
                        if occupied_count == 0 {
                            map_changed = true;
                            MapCell::Occupied
                        } else {
                            MapCell::Empty
                        }
                    }
                    MapCell::Floor => MapCell::Floor
                }
            }
        }

        map_to_process.swap(&map_to_update);
    }
    let map_to_process = map_to_process.borrow().clone();
    map_to_process
}

#[inline]
fn process_map_cell_found(cell: &MapCell, flag: &mut bool, occupied_count: &mut usize) {
    match cell {
        MapCell::Occupied => {
            *occupied_count += 1;
            *flag = true;
        }
        MapCell::Empty => *flag = true,
        MapCell::Floor => {}
    };
}

#[inline]
fn get_adjacent_cells(
    x: usize,
    row_above: &Option<&Vec<MapCell>>,
    current_row: &Vec<MapCell>,
    row_below: &Option<&Vec<MapCell>>,
) -> Vec<MapCell> {
    let mut adjacents: Vec<MapCell> = Vec::with_capacity(8);
    // Get adjacent cells from row above
    get_adjacent_from_row(&mut adjacents, x, &row_above);
    // Get adjacent cells to left and right
    if x > 0 {
        if let Some(left) = current_row.get(x - 1) {
            adjacents.push(left.clone());
        }
    }
    if let Some(right) = current_row.get(x + 1) {
        adjacents.push(right.clone());
    }
    // Get adjacent cells from row below
    get_adjacent_from_row(&mut adjacents, x, &row_below);

    adjacents
}

#[inline]
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
