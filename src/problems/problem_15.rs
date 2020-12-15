use std::collections::HashMap;

fn determine_2020th_step(input: &[usize], steps: usize) -> usize {
    let mut memory: HashMap<usize, usize> = input.iter().enumerate().map(|(idx, &number)| (number, idx + 1)).collect();
    let mut new_item = 0;

    for turn_counter in input.len() + 1..steps {
        new_item = match memory.insert(new_item, turn_counter) {
            None => 0,
            Some(existing) => turn_counter - existing
        };
    }
    new_item
}

#[cfg(test)]
mod tests {
    use crate::problems::problem_15::determine_2020th_step;
    use std::time::Instant;

    #[test]
    fn part_one() {
        let input: &[usize; 6] = &[14, 3, 1, 0, 9, 5];
        assert_eq!(614, determine_2020th_step(input, 2020));
    }

    #[test]
    fn part_two() {
        let now = Instant::now();
        let input: &[usize; 6] = &[14, 3, 1, 0, 9, 5];
        assert_eq!(1065, determine_2020th_step(input, 30000000));
        println!("{}", now.elapsed().as_millis());
    }
}
