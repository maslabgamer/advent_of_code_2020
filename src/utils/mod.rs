use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn read_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("No such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|line| line.expect("Could not parse line"))
        .collect()
}
