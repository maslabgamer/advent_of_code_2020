use std::f32::consts::PI;

#[derive(Clone, Debug)]
enum Direction {
    North,
    East,
    South,
    West
}

#[derive(Debug)]
struct Waypoint {
    x: i32,
    y: i32,
}

impl Waypoint {
    fn new(x: i32, y: i32) -> Self {
        Waypoint { x, y }
    }

    fn rotate(&mut self, degrees: f32) {
        // Trig functions expect radians
        let radians = degrees * (PI / 180f32);
        let new_x = (self.x as f32 * radians.cos()) - (self.y as f32 * radians.sin());
        let new_y = (self.y as f32 * radians.cos()) + (self.x as f32 * radians.sin());
        // Round to nearest int due to floating point nonsense
        self.x = new_x.round() as i32;
        self.y = new_y.round() as i32;
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

    pub fn parse_direction(&mut self, command: &str) {
        let mut command_iter = command.chars();
        let direction = command_iter.next().unwrap();
        let value: i32 = command_iter.into_iter().collect::<String>().parse::<i32>().unwrap();

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
                    'L' => waypoint.rotate(value as f32),
                    'R' => waypoint.rotate(-(value as f32)),
                    // Move ship relative to waypoint
                    'F' => {
                        self.vertical_distance_traveled += value * waypoint.y;
                        self.horizontal_distance_traveled += value * waypoint.x;
                    }
                    _ => panic!("Invalid command!")
                }
            }
        };
    }

    fn move_ship(&mut self, direction: Direction, distance: i32) {
        match direction {
            Direction::North => self.vertical_distance_traveled += distance,
            Direction::East => self.horizontal_distance_traveled += distance,
            Direction::South => self.vertical_distance_traveled -= distance,
            Direction::West => self.horizontal_distance_traveled -= distance
        };
    }
}
