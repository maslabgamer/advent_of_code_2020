use std::hash::{Hash, Hasher};
use std::collections::{HashMap, HashSet};
use std::borrow::Borrow;
use std::cell::Cell;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Coordinates {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Coordinates {
    fn new(x: i32, y: i32, z: i32, w: i32) -> Self {
        Coordinates { x, y, z, w }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct MapCell {
    coordinates: Coordinates,
    active: Cell<bool>,
    should_update: Cell<bool>, // If true, flip "active". Needed so that we can flip all needed after counting neighbors
    scanned: Cell<bool>
}

impl MapCell {
    fn new(x: i32, y: i32, z: i32, w: i32, state: bool) -> Self {
        MapCell {
            coordinates: Coordinates::new(x, y, z, w),
            active: Cell::new(state),
            should_update: Cell::new(false),
            scanned: Cell::new(false)
        }
    }

    fn determine_update_status(&self, cell_map: &HashMap<Coordinates, MapCell>) -> (i32, HashSet<MapCell>) {
        if self.scanned.get() {
            return (0, HashSet::new());
        }
        let mut active_count = 0;

        let coordinates = &self.coordinates;

        let mut new_cells: HashSet<MapCell> = HashSet::new();

        for x in coordinates.x - 1..coordinates.x + 2 {
            for y in coordinates.y - 1..coordinates.y + 2 {
                for z in coordinates.z - 1..coordinates.z + 2 {
                    for w in coordinates.w - 1..coordinates.w + 2 {
                        if coordinates.x == x && coordinates.y == y && coordinates.z == z && coordinates.w == w {
                            continue;
                        }
                        match cell_map.get(&Coordinates::new(x, y, z, w)) {
                            None => {
                                new_cells.insert(MapCell::new(x, y, z, w, false));
                            }
                            Some(cell) => {
                                if cell.active.get() {
                                    active_count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }

        if self.active.get() && (active_count != 2 && active_count != 3) {
            self.should_update.set(true);
        } else if !self.active.get() && active_count == 3 {
            self.should_update.set(true);
        }
        self.scanned.set(true);

        (active_count, new_cells)
    }

    fn get_active_state(&self) -> bool {
        // Finish any updates
        self.execute_update();
        self.active.get()
    }

    fn execute_update(&self) {
        if self.should_update.get() {
            self.active.set(!self.active.get());
        }
        self.should_update.set(false);
        self.scanned.set(false);
    }
}

impl Hash for MapCell {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.coordinates.hash(state);
    }
}

impl Borrow<Coordinates> for MapCell {
    fn borrow(&self) -> &Coordinates {
        &self.coordinates
    }
}

fn run_six_cycles(input: &[u8]) -> usize {
    let mut input = input;
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    let mut cells: HashMap<Coordinates, MapCell> = HashMap::with_capacity(2_560_000);
    println!("done initializing map");

    while let Some(current_char) = input.first() {
        match current_char {
            b'.' => {
                let new_cell = MapCell::new(x, y, 0, 0, false);
                cells.insert(new_cell.coordinates.clone(), new_cell);
                x += 1;
            }
            b'#' => {
                let new_cell = MapCell::new(x, y, 0, 0, true);
                cells.insert(new_cell.coordinates.clone(), new_cell);
                x += 1;
            }
            b'\n' => {
                x = 0;
                y += 1;
            }
            &_ => panic!("invalid char {}!", current_char)
        };

        input = &input[1..];
    }

    let mut new_cells: HashSet<MapCell> = HashSet::new();
    for _ in 0..6 {
        // Possible improvements
        // 1. Eagerly initialize map space. Currently I'm having to mutate the existing map with
        //    each cycle, and this means I have to iterate over it multiple times whenever I
        //    find a cell that doesn't already exist in it. It may be worth it to start by
        //    Initializing the map space with, say, 1k items to a "side" to start with
        //    Will prevent having to track new cells, add them into the map, and then iterate
        //    over the map again until we don't add anything else
        // 2. If that's not big enough, perhaps we iterate through the map to get a "list" of
        //    cells to check. If we have to add a cell into the map, push it to the end of the
        //    list of cells to process and add it into the map
        loop {
            let mut active_total_detected = 0;
            for cell in cells.values() {
                let (active_count, cells_to_add) = cell.determine_update_status(&cells);
                active_total_detected += active_count;
                new_cells.extend(cells_to_add);
            }
            // Add new cells
            new_cells.drain().for_each(|cell| {
                cells.insert(cell.coordinates.clone(), cell.clone());
            });
            if active_total_detected == 0 {
                break;
            }
        }

        cells.values().for_each(|cell| cell.execute_update());
    }

    cells.values()
        .filter(|cell| cell.get_active_state())
        .count()
}

#[cfg(test)]
mod tests {
    use crate::problems::problem_17::run_six_cycles;
    use std::time::Instant;

    #[test]
    fn part_1() {
        let input = include_bytes!("../../resources/problem_17_input.txt");
        assert_eq!(211, run_six_cycles(input));
    }

    #[test]
    fn part_2() {
        let now = Instant::now();
        let input = include_bytes!("../../resources/problem_17_input.txt");
        assert_eq!(1952, run_six_cycles(input));
        println!("{}", now.elapsed().as_millis());
    }
}