use std::collections::HashMap;

// Part 1
pub fn initialize_memory(input: &[u8]) -> u64 {
    let mut input = input;
    let mut mask: &[u8] = &[0];
    let mut memory: HashMap<i32, (u64, &[u8])> = HashMap::new();

    while let Some(_) = input.first() {
        let command = &input[1..4];
        if command == b"ask" {
            mask = &input[7..43];
            input = &input[44..];
        } else if command == b"em[" {
            let (memory_index, index_read_count) = lexical::parse_partial::<i32, _>(&input[4..]).unwrap();
            let (number, num_read_count) = lexical::parse_partial::<u64, _>(&input[8 + index_read_count..]).unwrap();
            memory.insert(memory_index, (number, mask.clone()));
            input = &input[9 + index_read_count + num_read_count..];
        }
    }
    memory.values().map(|(value, mask)| apply_mask(*value, mask)).sum()
}

#[inline]
fn apply_mask(mut number: u64, mask: &[u8]) -> u64 {
    mask.iter().enumerate()
        .filter(|(_, &c)| c != b'X')
        .for_each(|(i, m)| match m {
            b'0' => number &= !(1 << 35 - i),
            b'1' => number |= 1 << 35 - i,
            _ => panic!("Invalid mask char!")
        });
    number
}

// Part 2
pub fn initialize_memory_decoder(input: &[u8]) -> u64 {
    let mut input = input;
    let mut mask: &[u8] = &[0];
    let mut memory: HashMap<u64, u64> = HashMap::with_capacity(600 * 36);
    let mut memory_addresses: Vec<u64> = Vec::with_capacity(512);
    let mut total = 0;

    while let Some(_) = input.first() {
        let command = &input[1..4];
        if command == b"ask" {
            mask = &input[7..43]; // Mask is guaranteed to be at indices 7 to 42
            input = &input[44..]; // Update input to be after the following newline
        } else { // If it's not a "mask" command, it's a "mem"
            let (memory_index, index_read_count) = lexical::parse_partial::<u64, _>(&input[4..]).unwrap();
            let (number, num_read_count) = lexical::parse_partial::<u64, _>(&input[8 + index_read_count..]).unwrap();
            apply_mask_decoding(&mut memory_addresses, memory_index, mask);
            memory_addresses.drain(..).for_each(|address| {
                if let Some(existing) = memory.insert(address, number) {
                    total -= existing;
                }
                total += number;
            });
            input = &input[9 + index_read_count + num_read_count..]; // Update input to be after the following newline
        }
    }

    total
}

#[inline]
fn apply_mask_decoding(memory_addresses: &mut Vec<u64>, memory_idx: u64, mask: &[u8]) {
    let first = mask.first();
    match first {
        None => {
            memory_addresses.push(memory_idx)
        },
        Some(c) => {
            match c {
                b'0' => {
                    apply_mask_decoding(memory_addresses, memory_idx, &mask[1..]);
                },
                b'1' => {
                    let memory_idx = memory_idx | 1 << mask.len() - 1;
                    apply_mask_decoding(memory_addresses, memory_idx | 1 << mask.len() - 1, &mask[1..]);
                },
                b'X' => {
                    let bit_shifted = 1 << mask.len() - 1;
                    let new_memory_idx = memory_idx & !bit_shifted;
                    apply_mask_decoding(memory_addresses, new_memory_idx, &mask[1..]);
                    let new_memory_idx = memory_idx | bit_shifted;
                    apply_mask_decoding(memory_addresses, new_memory_idx, &mask[1..]);
                }
                &_ => panic!("Invalid char"),
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::problems::problem_14::{initialize_memory, initialize_memory_decoder};

    #[test]
    fn part_one() {
        let input = include_bytes!("../../resources/problem_14_input.txt");
        assert_eq!(17481577045893, initialize_memory(input));
    }

    #[test]
    fn part_two() {
        let input = include_bytes!("../../resources/problem_14_input.txt");
        assert_eq!(4160009892257, initialize_memory_decoder(input))
    }
}