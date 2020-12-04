use regex::Regex;

#[derive(Debug)]
enum UnitType {
    METRIC,
    IMPERIAL,
    INVALID
}

#[derive(Debug)]
pub(crate) struct Height {
    height: i32,
    units: UnitType
}

impl Height {
    pub(crate) fn new(height_str: &str) -> Option<Self> {
        let height_parse = Regex::new(r"(?P<height>\d*)(?P<unit>cm|in)").unwrap();

        match height_parse.captures(height_str) {
            None => None,
            Some(caps) => {
                let height = caps["height"].parse::<i32>().unwrap();
                let units = match &caps["unit"] {
                    "cm" => UnitType::METRIC,
                    "in" => UnitType::IMPERIAL,
                    _ => UnitType::INVALID,
                };

                Some(Height { height, units })
            }
        }
    }

    pub(crate) fn is_valid(&self) -> bool {
        match self.units {
            UnitType::METRIC => (150..=193).contains(&self.height),
            UnitType::IMPERIAL => (59..=76).contains(&self.height),
            UnitType::INVALID => false
        }
    }
}
