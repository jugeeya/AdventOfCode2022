use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};

pub fn file_to_lines(filepath: &str) -> Result<Lines<BufReader<File>>, std::io::Error> {
    let file = File::open(filepath)?;
    Ok(io::BufReader::new(file).lines())
}