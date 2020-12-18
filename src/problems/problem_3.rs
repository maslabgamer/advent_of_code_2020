fn process_map_and_movements(map: &Vec<&str>, slopes: &[(usize, usize)]) -> usize {
    slopes.iter()
        .map(|&movement| process_toboggan_movement(movement, &map))
        .product()
}

fn process_toboggan_movement(movement: (usize, usize), map: &Vec<&str>) -> usize {
    let map_width = map[0].len();

    (movement.1..map.len()).step_by(movement.1).enumerate()
        .filter(
            |(idx, y_pos)|
                map[*y_pos].chars()
                    .nth((movement.0 * (idx + 1)) % map_width)
                    .unwrap() == '#'
        ).count()
}

#[cfg(test)]
mod tests {
    use crate::problems::problem_3::process_map_and_movements;

    #[test]
    fn part_one() {
        let map: Vec<&str> = include_str!("../../resources/problem_3_input.txt").lines().collect();
        let slope = [(3, 1)];
        assert_eq!(272, process_map_and_movements(&map, &slope));
    }

    #[test]
    fn part_two() {
        let map: Vec<&str> = include_str!("../../resources/problem_3_input.txt").lines().collect();
        let slope = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        assert_eq!(3898725600, process_map_and_movements(&map, &slope));
    }
}
