use aoc_2020::utils::read_file;
use aoc_2020::processor::Processor;

fn main() {
    let program = read_file("resources/problem_8_input.txt");
    let mut processor = Processor::new(&program);
    let last_accumulator_val = processor.run_to_repeat();
    println!("Most recent accumulator val in program = {}", last_accumulator_val.get_accumulator());
    println!("Had to change line 378 to a nop");
}