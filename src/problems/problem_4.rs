use crate::passport::Passport;

fn process_problem(input: &str) -> usize {
    let mut passports = vec![Passport::new()];

    for line in input.lines() {
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

#[cfg(test)]
mod tests {
    use crate::problems::problem_4::process_problem;

    #[test]
    fn part_one() {
        let input = include_str!("../../resources/problem_4_input.txt");
        assert_eq!(172, process_problem(input));
    }
}
