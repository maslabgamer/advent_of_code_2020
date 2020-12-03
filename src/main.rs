mod utils;

use utils::read_file;
use regex::Regex;

fn main() {
    println!("Problem 1 solution: {}", problem_1());
    println!("Problem 2 part 1 solution: {}", problem_2_part_1());
    println!("Problem 2 part 2 solution: {}", problem_2_part_2());
    println!("Problem 3 solution: {}", problem_3());
}

fn problem_3() -> usize {
    let map = read_file("resources/problem_3_input.txt");

    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter()
        .map(|&movement| process_toboggan_movement(movement, &map))
        .product()
}

fn process_toboggan_movement(movement: (usize, usize), map: &Vec<String>) -> usize {
    let map_width = map[0].len();
    let mut trees_encountered = 0;

    for (idx, y_pos) in (movement.1..map.len()).step_by(movement.1).enumerate() {
        // move toboggan along x
        let toboggan_x = (movement.0 * (idx + 1)) % map_width;
        // Check for tree
        if map[y_pos].chars().nth(toboggan_x).unwrap() == '#' {
            trees_encountered += 1;
        }
    }
    trees_encountered
}

fn problem_2_part_2() -> i32 {
    let passwords = read_file("resources/problem_2_input.txt");
    let mut valid_pass_count: i32 = 0;

    // Fuck it, regex
    let re = Regex::new(r"(?P<min>\d*)-(?P<max>\d*) (?P<char>.): (?P<password>.*)").unwrap();

    for password in passwords {
        let caps = re.captures(password.as_ref()).unwrap();

        let first_index = caps["min"].parse::<usize>().unwrap() - 1;
        let second_index = caps["max"].parse::<usize>().unwrap() - 1;
        let character = caps["char"].chars().collect::<Vec<char>>()[0];
        let password = &caps["password"];

        let first_char = password.chars().nth(first_index).unwrap();
        let second_char = password.chars().nth(second_index).unwrap();
        if (first_char == character || second_char == character) && first_char != second_char {
            valid_pass_count += 1;
        }
    }

    valid_pass_count
}

fn problem_2_part_1() -> i32 {
    let passwords = read_file("resources/problem_2_input.txt");
    let mut valid_pass_count: i32 = 0;

    // Fuck it, regex
    let re = Regex::new(r"(?P<min>\d*)-(?P<max>\d*) (?P<char>.): (?P<password>.*)").unwrap();

    for password in passwords {
        let caps = re.captures(password.as_ref()).unwrap();

        let min = caps["min"].parse::<usize>().unwrap();
        let max = caps["max"].parse::<usize>().unwrap();
        let character = caps["char"].chars().collect::<Vec<char>>()[0];
        let password = &caps["password"];

        let counter = password.chars().filter(|&c| c == character).count();
        if counter >= min && counter <= max {
            valid_pass_count += 1;
        }
    }

    valid_pass_count
}

fn problem_1() -> i32 {
    let numbers = read_file("resources/problem_1_input.txt")
        .iter()
        .map(|number| number.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    for i in 0..numbers.len() - 2 {
        for j in i + 1..numbers.len() - 1 {
            for k in j + 1..numbers.len() {
                if numbers[i] + numbers[j] + numbers[k] == 2020 {
                    return numbers[i] * numbers[j] * numbers[k];
                }
            }
        }
    }
    0
}
