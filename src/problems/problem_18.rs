use std::ops::{Add, Mul};

pub fn evaluate_all_expressions(input: &[u8]) -> u64 {
    let mut input = input;

    let mut left_stack: Vec<u64> = vec![];
    let mut op_stack: Vec<fn(u64, u64) -> u64> = vec![];
    let mut left_side: u64 = 0;

    let mut current_operation: Option<fn(u64, u64) -> u64> = None;

    let mut answers: Vec<u64> = vec![];

    let mut total = 0;
    while let Some(first) = input.first() {
        match first {
            b'+' => current_operation = Some(u64::add),
            b'*' => current_operation = Some(u64::mul),
            b'(' => {
                match current_operation {
                    None => {
                        left_stack.push(0);
                        op_stack.push(u64::add);
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
            b' ' => {}
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

/// Addition is done before multiplication
pub fn swapped_precedence(input: &[u8]) -> u64 {
    let mut input = input;
    let mut total = 0;

    let mut output_stack: Vec<u8> = vec![];
    let mut operator_stack = vec![];
    let mut evaluation_stack: Vec<u64> = Vec::with_capacity(50);

    while let Some(first) = input.first() {
        match first {
            b'+' => {
                operator_stack.push(first);
            }
            b'*' => {
                while let Some(stack_operator) = operator_stack.last() {
                    if **stack_operator == b'+' {
                        output_stack.push(**stack_operator);
                        operator_stack.pop();
                    } else {
                        break;
                    }
                }
                operator_stack.push(first);
            }
            b'(' => operator_stack.push(first),
            b')' => {
                while let Some(stack_operator) = operator_stack.pop() {
                    if *stack_operator == b'(' {
                        break;
                    }
                    output_stack.push(*stack_operator);
                }
            }
            b'\n' => {
                while let Some(stack_operator) = operator_stack.pop() {
                    output_stack.push(*stack_operator);
                }
                for token in &output_stack {
                    match token {
                        b'+' => {
                            let left = evaluation_stack.pop().unwrap();
                            let right = evaluation_stack.pop().unwrap();
                            evaluation_stack.push(left + right);
                        },
                        b'*' => {
                            let left = evaluation_stack.pop().unwrap();
                            let right = evaluation_stack.pop().unwrap();
                            evaluation_stack.push(left * right);
                        },
                        _ => {
                            evaluation_stack.push((*token as char).to_digit(10).unwrap() as u64);
                        }
                    }
                }

                total += evaluation_stack.pop().unwrap();
            }
            b' ' => {}
            _ => output_stack.push(*first)
        }

        input = &input[1..];
    }

    total
}

#[cfg(test)]
mod tests {
    use crate::problems::problem_18::{evaluate_all_expressions, swapped_precedence};

    #[test]
    fn part_one() {
        let input = include_bytes!("../../resources/problem_18_input.txt");
        assert_eq!(3647606140187, evaluate_all_expressions(input));
    }

    #[test]
    fn part_two() {
        let input = include_bytes!("../../resources/problem_18_input.txt");
        assert_eq!(323802071857594, swapped_precedence(input));
    }
}