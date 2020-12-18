fn find_two(input: Vec<i32>) -> i32 {
    for i in 0..input.len() - 1 {
        for j in i + 1..input.len() {
            if input[i] + input[j] == 2020 {
                return input[i] * input[j];
            }
        }
    }
    0
}

fn find_three(input: Vec<i32>) -> i32 {
    for i in 0..input.len() - 2 {
        for j in i + 1..input.len() - 1 {
            for k in j + 1..input.len() {
                if input[i] + input[j] + input[k] == 2020 {
                    return input[i] * input[j] * input[k];
                }
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use crate::problems::problem_1::{find_two, find_three};

    #[test]
    fn part_one() {
        let input: Vec<i32> = include_str!("../../resources/problem_1_input.txt").lines()
            .map(|number| number.parse::<i32>().unwrap())
            .collect();
        assert_eq!(1009899, find_two(input));
    }

    #[test]
    fn part_two() {
        let input: Vec<i32> = include_str!("../../resources/problem_1_input.txt").lines()
            .map(|number| number.parse::<i32>().unwrap())
            .collect();
        assert_eq!(44211152, find_three(input));
    }
}