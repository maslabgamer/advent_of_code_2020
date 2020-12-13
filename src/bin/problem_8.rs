use aoc_2020::processor::Processor;

fn main() {
    let program: Vec<&str> = include_str!("../../resources/problem_8_input.txt").lines().collect();
    let mut processor = Processor::new(&program);
    let last_accumulator_val = processor.run_to_repeat();
    println!("Most recent accumulator val in program = {}", last_accumulator_val);
    println!("Had to change line 378 to a nop");
}