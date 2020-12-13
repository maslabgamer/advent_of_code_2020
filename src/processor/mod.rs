use std::str::FromStr;
use std::collections::HashSet;

/// Used to decode program commands
#[derive(Debug)]
enum Command {
    Accumulate,
    Jump,
    NoOperation,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "acc" => Ok(Command::Accumulate),
            "jmp" => Ok(Command::Jump),
            "nop" => Ok(Command::NoOperation),
            &_ => Err(())
        }
    }
}

// Decoded commands with args we will be executing
#[derive(Debug)]
struct Operation {
    command: Command,
    arg: i32
}

#[derive(Debug, Default)]
pub struct Processor {
    program: Vec<Operation>,
    program_counter: i32,
    accumulator: i32,
    executed_commands: HashSet<i32>,
}

impl Processor {
    pub fn new(program: &Vec<&str>) -> Self {
        let program = program.iter()
            .map(|op| Processor::parse_command(&op))
            .collect();

        Processor {
            program,
            ..Default::default()
        }
    }

    fn parse_command(operation: &str) -> Operation {
        let mut operation = operation.split_ascii_whitespace();
        let command = operation.next().unwrap().parse::<Command>().unwrap();
        let arg = operation.next().unwrap().parse::<i32>().unwrap();
        Operation { command, arg }
    }
    
    pub fn run_to_repeat(&mut self) -> i32 {
        while self.executed_commands.insert(self.program_counter) && self.program_counter < self.program.len() as i32 {
            let current_command = &self.program[self.program_counter as usize];

            match current_command.command {
                Command::Accumulate => {
                    self.accumulator += current_command.arg;
                    self.program_counter += 1;
                }
                Command::Jump => {
                    self.program_counter += current_command.arg;
                }
                Command::NoOperation => {
                    self.program_counter += 1;
                }
            }
        }

        // Program terminated normally if it naturally reached the end of the file
        if self.program_counter == self.program.len() as i32 {
            println!("Terminated normally!");
        }

        self.accumulator
    }
}
