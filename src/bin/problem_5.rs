use aoc_2020::utils::read_file;

fn main() {
    let all_passes = decode_all_passes();
    println!("Problem 5 part 1 solution: {}", all_passes.iter().max().unwrap());
    println!("Problem 5 part 2 solution: {}", find_missing_seat(&mut all_passes.clone()));
}

fn find_missing_seat(all_passes: &mut Vec<i32>) -> i32 {
    all_passes.sort();
    (all_passes[0]..*all_passes.last().unwrap())
        .into_iter()
        .filter(|&seat| !all_passes.contains(&(seat as i32)))
        .collect::<Vec<i32>>()[0]
}

fn decode_all_passes() -> Vec<i32> {
    let boarding_passes = read_file("resources/problem_5_input.txt");
    boarding_passes.iter()
        .map(|pass| decode_pass(&pass))
        .collect()
}

/// Return the Seat ID of the given boarding pass
fn decode_pass(boarding_pass: &str) -> i32 {
    // For each character, constrain the min and max
    // If char is an F, take the lower half. current_max will go halfway to current_min
    // If char is B, take the upper half. current_min will go to halfway to current_max
    let mut row_range: Vec<i32> = (0..128).collect();
    let mut column_range: Vec<i32> = (0..8).collect();
    for current_char in boarding_pass.chars() {
        if current_char == 'F' {
            row_range = row_range.iter().take(row_range.len() / 2).map(|&x| x).collect();
        } else if current_char == 'B' {
            row_range = row_range.iter().rev().take(row_range.len() / 2).map(|&x| x).rev().collect();
        } else if current_char == 'L' {
            column_range = column_range.iter().take(column_range.len() / 2).map(|&x| x).collect();
        } else if current_char == 'R' {
            column_range = column_range.iter().rev().take(column_range.len() / 2).map(|&x| x).rev().collect();
        }
    }

    (row_range[0] * 8) + column_range[0]
}