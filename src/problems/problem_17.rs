use std::collections::{HashMap, HashSet};

pub fn run_six_cycles(input: &[u8], consider_fourth_dimension: bool) -> usize {
    let neighbor_func = match consider_fourth_dimension {
        true => {
            |(x, y, z, w), cells: &mut HashMap<(i32, i32, i32, i32), u32>|
                for x in x - 1..x + 2 {
                    for y in y - 1..y + 2 {
                        for z in z - 1..z + 2 {
                            for w in w - 1..w + 2 {
                                *cells.entry((x, y, z, w)).or_insert(0) += 1;
                            }
                        }
                    }
                }
        }
        false => {
            |(x, y, z, _), cells: &mut HashMap<(i32, i32, i32, i32), u32>|
                for x in x - 1..x + 2 {
                    for y in y - 1..y + 2 {
                        for z in z - 1..z + 2 {
                            *cells.entry((x, y, z, 0)).or_insert(0) += 1;
                        }
                    }
                }
        }
    };

    let mut input = input;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut alive_cells: HashSet<(i32, i32, i32, i32)> = HashSet::with_capacity(2000);

    while let Some(current_char) = input.first() {
        match current_char {
            b'.' => {
                x += 1;
            }
            b'#' => {
                alive_cells.insert((x, y, 0, 0));
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

    let mut new_alive_set: HashSet<(i32, i32, i32, i32)> = HashSet::with_capacity(2000);
    for _ in 0..6 {
        let mut cells: HashMap<(i32, i32, i32, i32), u32> = HashMap::new();
        for alive_cell in &alive_cells {
            neighbor_func(*alive_cell, &mut cells);
        }

        for (k, v) in cells.iter() {
            if (alive_cells.contains(k) && *v == 4) || *v == 3 {
                new_alive_set.insert(k.clone());
            }
        }
        alive_cells = new_alive_set.clone();
        new_alive_set.clear();
    }

    alive_cells.len()
}

#[cfg(test)]
mod tests {
    use crate::problems::problem_17::run_six_cycles;

    #[test]
    fn part_1() {
        let input = include_bytes!("../../resources/problem_17_input.txt");
        assert_eq!(211, run_six_cycles(input, false));
    }

    #[test]
    fn part_2() {
        let input = include_bytes!("../../resources/problem_17_input.txt");
        assert_eq!(1952, run_six_cycles(input, true));
    }
}