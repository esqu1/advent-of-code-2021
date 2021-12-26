use std::fs::File;
use std::io::{self, BufRead};
use std::iter::Iterator;

pub fn read_file_for_day(day: i32) -> impl Iterator<Item = String> {
    let file = File::open(format!("inputs/input{:02}.txt", day)).unwrap();
    io::BufReader::new(file).lines().map(|x| x.unwrap())
}
