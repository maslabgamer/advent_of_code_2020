use std::collections::HashMap;

fn main() {
    let output = process_input();
    let part_1_answer = output.0.get(&1).unwrap() * output.0.get(&3).unwrap();
    println!("Part 1 answer = {}", part_1_answer);
    println!("Part 2 answer = {}", output.1);
}

fn process_input() -> (HashMap<usize, usize>, usize) {
    let mut adapters: Vec<usize> = include_str!("../../resources/problem_10_input.txt").lines()
        .map(|i| i.parse::<usize>().unwrap())
        .collect();
    // Add outlet in (assume 0 joltage)
    adapters.push(0);
    adapters.sort();
    // Add my device in (max adapter joltage plus 3)
    adapters.push(adapters.last().unwrap() + 3);

    // Part 1
    let joltage_differences: HashMap<usize, usize> = (1..adapters.len()).into_iter()
        .fold(HashMap::with_capacity(3), |mut j_diffs, idx| {
            *j_diffs.entry(adapters[idx] - adapters[idx - 1]).or_insert(0) += 1;
            j_diffs
        });

    // Part 2
    let mut paths: Vec<usize> = vec![0; adapters.len()];
    paths[0] = 1;
    for idx in 0..adapters.len() {
        let current_adapter = adapters[idx];
        let current_path = paths[idx];

        for offset in &[1, 2, 3] {
            if let Some(&possible_adapter) = adapters.get(idx + offset) {
                if possible_adapter <= current_adapter + 3 {
                    paths[idx + offset] += current_path;
                }
            }
        }
    }

    (joltage_differences, *paths.last().unwrap())
}