use aoc_2020::passport::Passport;

fn main() {
    println!("Problem 4 solution: {}", process_problem());
}

fn process_problem() -> usize {
    let passports_lines: Vec<&str> = include_str!("../../resources/problem_4_input.txt").lines().collect();

    let mut passports = vec![Passport::new()];

    for line in passports_lines {
        let line = line.trim();
        if line.len() == 0 {
            // Make a new passport here
            passports.push(Passport::new());
            continue;
        }

        let current_passport = passports.last_mut().unwrap();

        for attribute in line.split_ascii_whitespace() {
            let attribute: Vec<&str> = attribute.split(':').collect();
            current_passport.assign_attribute(attribute[0], &attribute[1]);
        }
    }

    passports.iter().filter(|&passport| passport.is_valid()).count()
}
