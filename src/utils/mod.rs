use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub fn get_lines(path: &str) -> Vec<String> {
    BufReader::new(File::open(path).expect("file not found")).lines().map(|x| x.unwrap()).collect()
}