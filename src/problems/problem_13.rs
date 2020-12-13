extern crate ring_algorithm;

use ring_algorithm::chinese_remainder_theorem;

const U32_MAX: u32 = u32::MAX;

pub fn find_earliest_bus(input: &[u8]) -> u32 {
    let (earliest_departure, read_count) = lexical::parse_partial::<u32, _>(&input[..]).unwrap();
    let mut input = &input[read_count + 1..]; // Make sure we skip the line break

    let mut earliest_bus = (U32_MAX, 0);
    while let Some(first_char) = input.first() {
        if *first_char == b'x' {
            input = &input[2..]; // Make sure we also skip the comma
        } else {
            let (bus_id, read_count) = lexical::parse_partial::<u32, _>(&input[..]).unwrap();
            // Now find how many times we have to multiply the "time" by to get to my departure time
            let departure_time = (bus_id * (earliest_departure / bus_id)) + bus_id;
            if departure_time < earliest_bus.0 {
                earliest_bus = (departure_time, bus_id);
            }
            input = &input[read_count + 1..]; // Next character guaranteed to be comma to skip
        }
    }
    earliest_bus.1 * (earliest_bus.0 - earliest_departure)
}

pub fn find_sequential_arrivals(input: &[u8]) -> i64 {
    // For this problem the first line is irrelevant
    let (_, read_count) = lexical::parse_partial::<u32, _>(&input[..]).unwrap();
    let mut input = &input[read_count + 1..]; // Make sure we skip the line break

    let mut place_idx = 0;
    let mut bus_ids: Vec<i64> = vec![];
    let mut time_differentials: Vec<i64> = vec![];
    while let Some(first_char) = input.first() {
        if *first_char == b'x' {
            input = &input[2..]; // Make sure we also skip the comma
        } else {
            let (bus_id, read_count) = lexical::parse_partial::<i64, _>(&input[..]).unwrap();
            bus_ids.push(bus_id);
            time_differentials.push(place_idx);
            input = &input[read_count + 1..]; // Next character guaranteed to be comma to skip
        }
        place_idx += 1;
    }

    bus_ids.iter().product::<i64>() - chinese_remainder_theorem::<i64>(&time_differentials[..], &bus_ids[..]).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::problems::problem_13::{find_earliest_bus, find_sequential_arrivals};

    #[test]
    fn part_one() {
        let input = include_bytes!("../../resources/problem_13_input.txt");
        assert_eq!(2215, find_earliest_bus(input));
    }

    #[test]
    fn part_two() {
        let input = include_bytes!("../../resources/problem_13_input.txt");
        assert_eq!(1058443396696792, find_sequential_arrivals(input));
    }
}