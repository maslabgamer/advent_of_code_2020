fn main() {
    let all_passes = decode_all_passes();
    println!("Problem 5 part 1 solution: {}", all_passes.iter().max().unwrap());
    println!("Problem 5 part 2 solution: {}", find_missing_seat(&mut all_passes.clone()));
}

fn find_missing_seat(all_passes: &mut Vec<i32>) -> i32 {
    all_passes.sort();
    (all_passes[0]..*all_passes.last().unwrap()).into_iter()
        .filter(|&seat| !all_passes.contains(&(seat as i32)))
        .last().unwrap()
}

fn decode_all_passes() -> Vec<i32> {
    include_str!("../../resources/problem_5_input.txt").lines()
        .map(|pass| decode_pass(&pass))
        .collect()
}

fn decode_pass(boarding_pass: &str) -> i32 {
    let pass: String = boarding_pass.chars()
        .map(|c| (['B', 'R'].contains(&c) as i32).to_string())
        .collect();
    i32::from_str_radix(&pass, 2).unwrap()
}