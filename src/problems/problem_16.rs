use std::collections::{HashSet, HashMap};
use std::ops::Range;

pub fn find_invalid_values(input: &str) -> u32 {
    let input_iter = &mut input.lines();

    let mut fields: HashSet<Range<u32>> = HashSet::with_capacity(100);
    // First, parse the actual fields
    while let Some(line) = input_iter.next() {
        if line.len() == 0 {
            break;
        }
        let mut line_split = line.split(':').skip(1);
        let mut field_ranges = line_split.next().unwrap().trim().split(" or ");
        fields.insert(parse_range(field_ranges.next().unwrap()));
        fields.insert(parse_range(field_ranges.next().unwrap()));
    }

    // We are ignoring our ticket for this problem
    if input_iter.advance_by(4).is_err() {
        panic!("couldn't skip four elements for some reason (input malformed?)")
    }

    let mut answer = 0;

    while let Some(line) = input_iter.next() {
        for number in line.split(',').map(|i| i.parse::<u32>().unwrap()) {
            if !fields.iter().any(|range| range.contains(&number)) {
                answer += number;
            }
        }
    }

    answer
}

pub fn parse_valid_tickets(input: &str) -> u64 {
    let input_iter = &mut input.lines();

    let mut fields: HashMap<&str, (Range<u32>, Range<u32>)> = HashMap::with_capacity(100);
    // First, parse the actual fields
    while let Some(line) = input_iter.next() {
        if line.len() == 0 {
            break;
        }
        let mut line_split = line.split(':');
        let field_name = line_split.next().unwrap();
        let mut field_ranges = line_split.next().unwrap().trim().split(" or ");
        fields.insert(
            field_name,
            (parse_range(field_ranges.next().unwrap()), parse_range(field_ranges.next().unwrap()))
        );
    }
    // We just want my ticket info
    input_iter.next();
    let my_ticket: Vec<u64> = input_iter.next().unwrap().split(',').map(|i| i.parse::<u64>().unwrap()).collect();
    input_iter.next();
    input_iter.next();

    let mut valid_tickets: Vec<Vec<u32>> = Vec::with_capacity(280);
    'outer: while let Some(line) = input_iter.next() {
        let line: Vec<u32> = line.split(',').map(|i| i.parse::<u32>().unwrap()).collect();
        for number in &line {
            if !fields.values().any(|(left_range, right_range)| left_range.contains(&number) || right_range.contains(&number)) {
                continue 'outer;
            }
        }
        valid_tickets.push(line);
    }

    // So basically, we know there is one answer.
    // This means that if we check all indices 'i' for all lines against our 'fields' hashmap
    // there will be an index 'i' that fits into a single field
    // Remove that field and iterate again, and there will be another field that matches appropriately
    let mut answer: u64 = 1;
    let mut indices_to_ignore: HashSet<usize> = HashSet::with_capacity(my_ticket.len());
    let mut matching_field_counts: HashMap<&str, Vec<usize>> = HashMap::with_capacity(my_ticket.len());
    while !fields.is_empty() {
        for idx in 0..my_ticket.len() {
            if !indices_to_ignore.contains(&idx) {
                let nested_elements: Vec<u32> = valid_tickets.iter().map(|ticket| ticket[idx]).collect();
                for (key, (left_range, right_range)) in &fields {
                    if nested_elements.iter().all(|number| left_range.contains(number) || right_range.contains(number)) {
                        matching_field_counts.entry(key).or_insert(vec![]).push(idx);
                    }
                }
            }
        }

        for (removal_field, idx_to_ignore) in matching_field_counts.drain().filter(|(_, v)| v.len() == 1) {
            if removal_field.starts_with("departure") {
                answer *= my_ticket[idx_to_ignore[0]];
            }
            indices_to_ignore.insert(idx_to_ignore[0]);
            fields.remove(removal_field);
        }
    }

    answer
}

#[inline]
fn parse_range(range_str: &str) -> Range<u32> {
    let mut range_str = range_str.split('-');
    let left = range_str.next().unwrap().parse::<u32>().unwrap();
    let right = range_str.next().unwrap().parse::<u32>().unwrap();
    left..right + 1
}

#[cfg(test)]
mod tests {
    use crate::problems::problem_16::{find_invalid_values, parse_valid_tickets};

    #[test]
    fn part_one() {
        let input = include_str!("../../resources/problem_16_input.txt");
        assert_eq!(19240, find_invalid_values(input))
    }

    #[test]
    fn part_two() {
        let input = include_str!("../../resources/problem_16_input.txt");
        assert_eq!(21095351239483, parse_valid_tickets(input))
    }
}
