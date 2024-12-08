use std::env;
use std::fs;
use std::iter::zip;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: main.rs <input>");
    }
    let input_file = &args[1];
    let contents = fs::read_to_string(input_file).expect("Should have been able to read the file");
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = contents
        .split("\r\n")
        .map(|line| {
            let nums: Vec<i32> = line
                .split("   ")
                .map(|x| x.parse::<i32>().expect("Not a number!"))
                .collect();
            (nums[0], nums[1])
        })
        .unzip();
    left.sort();
    right.sort();

    // Part 1
    let differences = zip(left.clone(), right.clone()).map(|(l, r)| (l.abs_diff(r)));
    let total: u32 = differences.sum();
    println!("Sum of Differences: {total}");

    // Part 2
    let similarities = left
        .iter()
        .map(|x| x * right.iter().filter(|y| x == *y).count() as i32);
    let total: i32 = similarities.sum();
    println!("Sum of Similarities: {total}");
}
