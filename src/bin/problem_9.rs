fn main() {
    let response = process_input(25);
    println!("Problem 9 part 1 solution: {}", response.first_invalid);
    println!("Problem 9 part 2 solution: {}", response.min_max_sum);
}

struct Answer {
    first_invalid: u64,
    min_max_sum: u64
}

fn process_input(preamble_size: usize) -> Answer {
    let input: Vec<u64> = include_str!("../../resources/problem_9_input.txt").lines()
        .map(|i| i.parse::<u64>().unwrap())
        .collect();

    let invalid_idx = (preamble_size..input.len()).into_iter()
        .filter(|&idx| !find_sum(&input[idx - preamble_size..idx], input[idx]))
        .next().unwrap();
    let first_invalid = input[invalid_idx];

    // Now we have the first invalid number, search the list of numbers
    // for a contiguous series of at least two that sums to the invalid one
    let mut contiguous_slice_size = 2;
    let mut min_max_sum = 0;
    while contiguous_slice_size < input.len() && min_max_sum == 0 {
        for idx in 0..input.len() - (contiguous_slice_size - 1) {
            let slice = &input[idx..idx + contiguous_slice_size];
            if slice.iter().sum::<u64>() == first_invalid {
                min_max_sum = *slice.iter().min().unwrap() + *slice.iter().max().unwrap();
                break;
            }
        }
        contiguous_slice_size += 1;
    }

    Answer { first_invalid, min_max_sum }
}

fn find_sum(preamble: &[u64], current_number: u64) -> bool {
    for i in 0..preamble.len() - 1 {
        for j in i + 1..preamble.len() {
            if preamble[i] + preamble[j] == current_number {
                return true;
            }
        }
    }
    false
}

