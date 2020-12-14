use std::collections::HashMap;

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

#[cfg(test)]
mod tests {
    use crate::problems::problem_14::initialize_memory;

    #[test]
    fn part_one() {
        let input = include_bytes!("../../resources/problem_14_input.txt");
        assert_eq!(17481577045893, initialize_memory(input))
    }

    #[test]
    fn part_two() {

    }
}