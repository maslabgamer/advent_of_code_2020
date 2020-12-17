use std::collections::{HashMap, HashSet};

fn run_six_cycles(input: &[u8]) -> usize {
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

    for _ in 0..6 {
        let mut new_alive_set: HashSet<(i32, i32, i32, i32)> = HashSet::with_capacity(2000);
        let mut cells: HashMap<(i32, i32, i32, i32), u32> = HashMap::new();
        for alive_cell in &alive_cells {
            for x in alive_cell.0 - 1..alive_cell.0 + 2 {
                for y in alive_cell.1 - 1..alive_cell.1 + 2 {
                    for z in alive_cell.2 - 1..alive_cell.2 + 2 {
                        for w in alive_cell.3 - 1..alive_cell.3 + 2 {
                            *cells.entry((x, y, z, w)).or_insert(0) += 1;
                        }
                    }
                }
            }
        }

        for (k, v) in cells.iter() {
            if (alive_cells.contains(k) && *v == 4) || *v == 3 {
                new_alive_set.insert(k.clone());
            }
        }
        alive_cells = new_alive_set.clone();
    }

    alive_cells.len()
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