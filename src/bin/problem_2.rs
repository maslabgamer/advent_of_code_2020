use aoc_2020::utils::read_file;
use regex::Regex;

#[derive(PartialEq)]
enum FuncSelect {
    Part1,
    Part2
}

fn main() {
    println!("Problem 2 part 1 solution: {}", problem_2(FuncSelect::Part1));
    println!("Problem 2 part 2 solution: {}", problem_2(FuncSelect::Part2));
}

fn problem_2(function_selection: FuncSelect) -> i32 {
    let passwords = read_file("resources/problem_2_input.txt");
    let mut valid_pass_count: i32 = 0;

    let re = Regex::new(r"(?P<min>\d*)-(?P<max>\d*) (?P<char>.): (?P<password>.*)").unwrap();

    for password in passwords {
        let caps = re.captures(password.as_ref()).unwrap();

        let left_num = caps["min"].parse::<usize>().unwrap();
        let right_num = caps["max"].parse::<usize>().unwrap();
        let character = caps["char"].chars().collect::<Vec<char>>()[0];
        let password = &caps["password"];

        if function_selection == FuncSelect::Part1 {
            let counter = password.chars().filter(|&c| c == character).count();
            if (left_num..=right_num).contains(&counter) {
                valid_pass_count += 1;
            }
        } else {
            let first_char = password.chars().nth(left_num - 1).unwrap();
            let second_char = password.chars().nth(right_num - 1).unwrap();
            if (first_char == character || second_char == character) && first_char != second_char {
                valid_pass_count += 1;
            }
        }
    }

    valid_pass_count
}
