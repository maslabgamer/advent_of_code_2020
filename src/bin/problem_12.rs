use aoc_2020::utils::read_file;
use aoc_2020::ship::Ship;

fn main() {
    println!("Part 1");
    let directions = read_file("resources/problem_12_input.txt");
    let mut ship = Ship::new(false);
    directions.iter().for_each(|direction| ship.parse_direction(direction));
    let manhattan_distance = ship.get_total_distance_traveled();
    println!("Manhattan distance from start = {}", manhattan_distance);

    println!("Part 2");
    let mut ship = Ship::new(true);
    directions.iter().for_each(|direction| ship.parse_direction(direction));
    let manhattan_distance = ship.get_total_distance_traveled();
    println!("Manhattan distance from start = {}", manhattan_distance);
}
