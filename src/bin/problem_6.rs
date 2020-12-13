#![feature(iterator_fold_self)]
use std::collections::HashSet;

fn main() {
    println!("Problem 6 solution: {}", process_customs());
}

fn process_customs() -> usize {
    let customs_file = include_str!("../../resources/problem_6_input.txt").lines();
    let mut groups_answers: Vec<HashSet<char>> = vec![];
    let mut current_group: Vec<HashSet<char>> = vec![];

    for customs in customs_file {
        if customs.is_empty() {
            drain_passes(&mut groups_answers, &mut current_group);
        } else {
            current_group.push(customs.chars().collect());
        }
    }
    if !current_group.is_empty() {
        drain_passes(&mut groups_answers, &mut current_group);
    }

    groups_answers.iter().map(|answers| answers.len()).sum()
}

/// Drain to_drain into the container
/// to_drain - Vector containing all of a group's answers
/// container - Vector containing the intersection of all the group's answers
/// Don't return anything, just push it directly onto the container
fn drain_passes(container: &mut Vec<HashSet<char>>, to_drain: &mut Vec<HashSet<char>>) {
    let mut all_yes = to_drain.drain(..)
        .fold_first(
            |a, b|
                a.intersection(&b).into_iter()
                    .map(|c| *c).collect()
        ).unwrap();
    container.push(all_yes.drain().collect());
}
