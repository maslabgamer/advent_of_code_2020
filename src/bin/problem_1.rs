use aoc_2020::utils::read_file;

fn main() {
    println!("Problem 1 solution: {}", process_input());
}

fn process_input() -> i32 {
    let numbers: Vec<i32> = read_file("resources/problem_1_input.txt")
        .iter()
        .map(|number| number.parse::<i32>().unwrap())
        .collect();
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
