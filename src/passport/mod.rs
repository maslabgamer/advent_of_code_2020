mod height;

use height::Height;

const VALID_HAIR_COLOR_CHARS: [char; 16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];
const VALID_PASSPORT_CHARS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

#[derive(Debug)]
pub struct Passport {
    birth_year: Option<i32>,
    issue_year: Option<i32>,
    expiration_year: Option<i32>,
    height: Option<Height>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

impl Passport {
    pub fn new() -> Self {
        Passport {
            birth_year: None,
            issue_year: None,
            expiration_year: None,
            height: None,
            hair_color: None,
            eye_color: None,
            passport_id: None,
            country_id: None,
        }
    }

    pub fn assign_attribute(&mut self, key: &str, value: &str) {
        match key {
            "byr" => self.birth_year = Some(value.parse::<i32>().unwrap()),
            "iyr" => self.issue_year = Some(value.parse::<i32>().unwrap()),
            "eyr" => self.expiration_year = Some(value.parse::<i32>().unwrap()),
            "hgt" => self.height = Height::new(value),
            "hcl" => self.hair_color = Some(value.parse().unwrap()),
            "ecl" => self.eye_color = Some(value.parse().unwrap()),
            "pid" => self.passport_id = Some(value.parse().unwrap()),
            "cid" => self.country_id = Some(value.parse().unwrap()),
            &_ => {}
        }
    }

    pub fn is_valid(&self) -> bool {
        Passport::validate_field(&self.birth_year, |&birth_year| (1920..=2002).contains(&birth_year))
            && Passport::validate_field(&self.issue_year, |&issue_year| (2010..=2020).contains(&issue_year))
            && Passport::validate_field(&self.expiration_year, |&expiration_year| (2020..=2030).contains(&expiration_year))
            && Passport::validate_field(&self.height, |height| height.is_valid())
            && Passport::validate_field(&self.hair_color, |hair_color| Passport::validate_hair_color(&hair_color))
            && Passport::validate_field(&self.eye_color, |eye_color| EYE_COLORS.contains(&&**eye_color))
            && Passport::validate_field(&self.passport_id, |passport_id| Passport::validate_passport_id(&passport_id))
    }

    fn validate_field<T, U>(field: &Option<T>, valid_func: U) -> bool where U: Fn(&T) -> bool {
        match field {
            None => false,
            Some(field_val) => valid_func(field_val)
        }
    }

    fn validate_hair_color(hair_color: &str) -> bool {
        if !hair_color.starts_with('#') {
            return false;
        }
        hair_color.chars().skip(1).all(|c| VALID_HAIR_COLOR_CHARS.contains(&c))
    }

    fn validate_passport_id(passport_id: &str) -> bool {
        if passport_id.len() != 9 {
            return false;
        }
        passport_id.chars().all(|c| VALID_PASSPORT_CHARS.contains(&c))
    }
}
