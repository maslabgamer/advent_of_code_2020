use aoc_2020::utils::read_file;

fn main() {
    println!("Problem 3 solution: {}", process_map_and_movements());
}

fn process_map_and_movements() -> usize {
    let map = read_file("resources/problem_3_input.txt");

    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter()
        .map(|&movement| process_toboggan_movement(movement, &map))
        .product()
}

fn process_toboggan_movement(movement: (usize, usize), map: &Vec<String>) -> usize {
    let map_width = map[0].len();

    (movement.1..map.len()).step_by(movement.1).enumerate()
        .filter(
            |(idx, y_pos)|
                map[*y_pos].chars()
                    .nth((movement.0 * (idx + 1)) % map_width)
                    .unwrap() == '#'
        ).count()
}
