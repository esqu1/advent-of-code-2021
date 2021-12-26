use crate::utils::read_file_for_day;

fn solve_helper(input: &Vec<i32>, window_size: usize) -> i32 {
    let mut prev = 0;
    let mut increases = 0;
    for (idx, val) in input.iter().enumerate() {
        if idx < window_size {
            prev += val;
        } else {
            let new_window = prev + val - input[idx - window_size];
            if new_window > prev {
                increases += 1;
            }
            prev = new_window;
        }
    }
    increases
}

pub fn solve() {
    let lines = read_file_for_day(1);
    let mut input = vec![];
    for line in lines {
        input.push(line.parse::<i32>().unwrap());
    }

    // Part 1
    println!("Day 1 Part 1: {}", solve_helper(&input, 1));

    // Part 2
    println!("Day 1 Part 2: {}", solve_helper(&input, 3));
}
