use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename = "input.txt";
    let mut total = 0;

    // Cache for previous characters
    let mut buffer: String = String::new();

    // list of all numbers as strings
    let numbers: Vec<&str> = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    if let Ok(lines) = read_lines(filename) {
        // loop through lines
        for line in lines {
            let line = match line {
                Ok(l) => l,
                Err(_) => {
                    continue;
                }
            };

            // vector of numbers for each row
            let mut nums: Vec<u32> = Vec::new();

            for c in line.chars() {
                if c.is_digit(10) {
                    buffer.clear();

                    let num: u32 = match c.to_digit(10) {
                        None => continue,
                        Some(n) => n,
                    };

                    nums.push(num);
                } else {
                    buffer += &c.to_string();

                    match numbers.iter().position(|n| buffer.ends_with(n)) {
                        Some(n) => nums.push(n as u32),
                        None => continue,
                    }
                }
            }

            let value = 10 * nums[0] + nums[nums.len() - 1];
            total += value;
        }
    }
    println!("{}", total);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
