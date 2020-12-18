fn evaluate_all_expressions(input: &[u8]) -> u64 {
    let mut input = input;

    let mut left_stack: Vec<u64> = vec![];
    let mut op_stack: Vec<fn(u64, u64) -> u64> = vec![];
    let mut left_side: u64 = 0;

    let mut current_operation: Option<fn(u64, u64) -> u64> = None;

    let mut answers: Vec<u64> = vec![];

    let mut total = 0;
    while let Some(first) = input.first() {
        match first {
            b'+' => current_operation = Some(|a, b| a + b),
            b'-' => current_operation = Some(|a, b| a - b),
            b'*' => current_operation = Some(|a, b| a * b),
            b'(' => {
                match current_operation {
                    None => {
                        left_stack.push(0);
                        op_stack.push(|a, b| a + b);
                    }
                    Some(some_op) => {
                        left_stack.push(left_side);
                        op_stack.push(some_op);
                        current_operation = None;
                    }
                }
                left_side = 0;
            }
            b')' => {
                if let Some(left_op) = op_stack.pop() {
                    left_side = left_op(left_side, left_stack.pop().unwrap());
                    current_operation = Some(left_op);
                }
            }
            b'\n' => {
                total += left_side;
                answers.push(left_side);
                left_side = 0;
                current_operation = None;
            }
            b' ' => {},
            _ => {
                let (right_side, _) = lexical::parse_partial::<u64, _>(&input).unwrap();
                left_side = match current_operation {
                    None => right_side,
                    Some(current_operation) => current_operation(left_side, right_side),
                };
            }
        }
        input = &input[1..];
    }
    total
}

#[cfg(test)]
mod tests {
    use crate::problems::problem_18::evaluate_all_expressions;

    #[test]
    fn part_one() {
        let input = include_bytes!("../../resources/problem_18_input.txt");
        println!("{}", evaluate_all_expressions(input));
    }
}