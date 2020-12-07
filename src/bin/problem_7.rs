use aoc_2020::utils::read_file;
use std::collections::{HashMap, HashSet};
use regex::Regex;

fn main() {
    let bag_rules = setup_bag_rules();
    let contains_gold = get_all_gold_contains(&bag_rules);
    // Part 1
    println!("{} bags can have a shiny gold bag in them.", contains_gold.len());

    // Part 2
    let mut contains = bag_rules.get("shiny gold").unwrap().clone();
    let mut total = 0;

    while let Some(current_bag) = contains.pop() {
        total += current_bag.1;
        if let Some(new_bags) = bag_rules.get(&current_bag.0) {
            let new_bags: Vec<(String, i32)> = new_bags.iter()
                .map(|(b, i)| (b.to_string(), i * current_bag.1))
                .collect();
            contains.extend(new_bags);
        }
    }

    println!("Golden bag contains {} bags.", total);
}

fn get_all_gold_contains(bag_rules: &HashMap<String, Vec<(String, i32)>>) -> HashSet<String> {
    let mut contains_gold = HashSet::new();
    for (key, value) in bag_rules.iter() {
        // Depth-first search to find either
        // 1. shiny gold bag in list of possible values
        // 2. item we already know can contain a shiny gold bag
        let mut values = value.clone();
        while let Some(current_bag) = values.pop() {
            let current_bag = current_bag.0;
            if current_bag.starts_with("shiny gold") || contains_gold.contains(&current_bag) {
                contains_gold.insert(key.clone());
                break;
            } else if let Some(new_bags) = bag_rules.get(&current_bag) {
                values.extend(new_bags.clone());
            }
        }
    }

    contains_gold
}

fn setup_bag_rules() -> HashMap<String, Vec<(String, i32)>> {
    let bag_rules_file = read_file("resources/problem_7_input.txt");
    let mut bag_rules: HashMap<String, Vec<(String, i32)>> = HashMap::with_capacity(bag_rules_file.len());
    let re = Regex::new(r"(?P<number>\d) (?P<color>.*) bag").unwrap();

    for rule in bag_rules_file {
        let mut rule = rule.split("contain");
        let outer_bag = rule.next().unwrap().replace("bags", "");
        let contains = rule.next().unwrap().trim().strip_suffix('.').unwrap();

        let bags_contained: Vec<(String, i32)> = contains.split(',')
            .filter(|&b| b.trim() != "no other bags")
            .map(|b| {
                let caps = re.captures(b).unwrap();
                (caps["color"].to_string(), caps["number"].parse::<i32>().unwrap())
            })
            .collect();

        bag_rules.insert(outer_bag.trim().to_string(), bags_contained);
    }
    bag_rules
}
