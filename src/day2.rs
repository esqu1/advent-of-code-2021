use crate::utils::read_file_for_day;
use std::iter::Iterator;

fn solve_helper(lines: impl Iterator<Item = String>, track_aim: bool) -> i32 {
    let mut depth = 0;
    let mut hori = 0;
    let mut aim = 0;
    for line in lines {
        let delim = line.find(' ').unwrap();
        let (command, length_str) = line.split_at(delim);
        let length = length_str.trim().parse::<i32>().unwrap();
        match command {
            "forward" => {
                hori += length;
                if track_aim {
                    depth += aim * length;
                }
            }
            "down" => {
                if track_aim {
                    aim += length;
                } else {
                    depth += length;
                }
            }
            "up" => {
                if track_aim {
                    aim -= length;
                } else {
                    depth -= length;
                }
            }
            _ => {
                panic!("undefined direction");
            }
        }
    }

    depth * hori
}

pub fn solve() {
    let lines = read_file_for_day(2);
    println!("Day 2 Part 1: {}", solve_helper(lines, false));

    // reading in file twice to avoid loading file into memory... need a better
    // story for encapsulation while saving memory
    let lines = read_file_for_day(2);
    println!("Day 2 Part 2: {}", solve_helper(lines, true));
}
