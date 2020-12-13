pub fn run_base() {
    let directions = include_bytes!("../../resources/problem_12_input.txt");
    let mut ship = Ship::new(false);
    ship.process_commands(directions);
    let manhattan_distance = ship.get_total_distance_traveled();
    assert_eq!(962, manhattan_distance);

    let mut ship = Ship::new(true);
    ship.process_commands(directions);
    let manhattan_distance = ship.get_total_distance_traveled();
    assert_eq!(56135, manhattan_distance);
}

#[derive(Clone, Debug)]
enum Direction {
    North,
    East,
    South,
    West
}

#[derive(Debug)]
struct Waypoint {
    x: i16,
    y: i16,
}

impl Waypoint {
    fn new(x: i16, y: i16) -> Self {
        Waypoint { x, y }
    }

    fn rotate(&mut self, degrees: i16) {
        let sin_res = match degrees {
            90 | -270 => 1,
            180 | -180 => 0,
            270 | -90 => -1,
            _ => panic!("Invalid degrees amount!")
        };
        let cos_res = match degrees {
            90 | -90 | 270 | -270 => 0,
            180 | -180 => -1,
            _ => panic!("Invalid degrees amount!")
        };
        // Trig functions expect radians
        let new_x = (self.x * cos_res) - (self.y * sin_res);
        let new_y = (self.y * cos_res) + (self.x * sin_res);
        // Round to nearest int due to floating point nonsense
        self.x = new_x;
        self.y = new_y;
    }
}

pub struct Ship {
    // Current heading will be first item in this vector (rotate vector to change heading)
    headings: Vec<Direction>,
    waypoint: Option<Waypoint>,
    vertical_distance_traveled: i32,
    horizontal_distance_traveled: i32,
}

impl Ship {
    pub fn new(has_waypoint: bool) -> Self {
        let waypoint = match has_waypoint {
            true => Some(Waypoint::new(10 , 1)),
            false => None
        };
        Ship {
            headings: vec![Direction::East, Direction::North, Direction::West, Direction::South],
            waypoint,
            vertical_distance_traveled: 0,
            horizontal_distance_traveled: 0,
        }
    }

    pub fn get_total_distance_traveled(&self) -> i32 {
        self.horizontal_distance_traveled.abs() + self.vertical_distance_traveled.abs()
    }

    pub fn process_commands(&mut self, command_list: &[u8]) {
        let mut scan_idx = 0;
        while scan_idx < command_list.len() {
            let command: char = command_list[scan_idx].into();
            if command == '\n' {
                scan_idx += 1;
                continue;
            }
            let (value, read_count) = lexical::parse_partial::<i16, _>(&command_list[scan_idx + 1..]).unwrap();
            scan_idx += read_count + 1;

            self.parse_direction(command, value);
        }
    }

    pub fn parse_direction(&mut self, direction: char, value: i16) {
        match &mut self.waypoint {
            None => {
                match direction {
                    // First handle exact direction instructions
                    'N' => self.move_ship(Direction::North, value),
                    'S' => self.move_ship(Direction::South, value),
                    'E' => self.move_ship(Direction::East, value),
                    'W' => self.move_ship(Direction::West, value),
                    // Now handle turns
                    'L' => self.headings.rotate_left((value / 90) as usize),
                    'R' => self.headings.rotate_right((value / 90) as usize),
                    // Handle forward
                    'F' => self.move_ship(self.headings[0].clone(), value),
                    _ => panic!("Invalid command!")
                }
            }
            Some(waypoint) => {
                match direction {
                    // First handle exact direction instructions
                    'N' => waypoint.y += value,
                    'S' => waypoint.y -= value,
                    'E' => waypoint.x += value,
                    'W' => waypoint.x -= value,
                    // Now handle rotations
                    'L' => waypoint.rotate(value),
                    'R' => waypoint.rotate(-value),
                    // Move ship relative to waypoint
                    'F' => {
                        self.vertical_distance_traveled += (value * waypoint.y) as i32;
                        self.horizontal_distance_traveled += (value * waypoint.x) as i32;
                    }
                    _ => panic!("Invalid command!")
                }
            }
        };
    }

    fn move_ship(&mut self, direction: Direction, distance: i16) {
        match direction {
            Direction::North => self.vertical_distance_traveled += distance as i32,
            Direction::East => self.horizontal_distance_traveled += distance as i32,
            Direction::South => self.vertical_distance_traveled -= distance as i32,
            Direction::West => self.horizontal_distance_traveled -= distance as i32
        };
    }
}
