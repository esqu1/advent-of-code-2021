use crate::utils::read_file_for_day;
use std::iter::Iterator;

type Output = u32;

pub fn solve_helper(lines: impl Iterator<Item = String>) -> Output {
    let mut relative_bits = [0 as i32; 12];
    for line in lines {
        for (idx, c) in line.chars().enumerate() {
            relative_bits[idx] += if c == '1' { 1 } else { -1 };
        }
    }
    let mut gamma = 0 as Output;
    for bit in relative_bits {
        gamma += if bit > 0 { 1 } else { 0 };
        gamma <<= 1;
    }
    gamma >>= 1;

    let epsilon = gamma ^ ((1 << 12) - 1);
    gamma * epsilon
}

pub fn solve_helper2(lines: impl Iterator<Item = String>) -> Output {

}

pub fn solve() {
    let lines = read_file_for_day(3);

    println!("Day 3 Part 1: {}", solve_helper(lines));
}
