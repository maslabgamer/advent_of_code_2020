use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};

/// Returns the contents of a file as a Vector of Strings where each String is a line in the file
/// These are expected to be files from the resources folder, so no error handling is being done
pub fn read_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("No such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|line| line.expect("Could not parse line"))
        .collect()
}
