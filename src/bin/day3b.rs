use std::cmp::{max, min};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename = "input.txt";
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut total: u32 = 0;

    if let Ok(lines) = read_lines(filename) {
        // loop through lines
        for line in lines {
            let line = match line {
                Ok(l) => l,
                Err(_) => {
                    continue;
                }
            };
            map.push(line.chars().collect());
        }
    }
    for r in 0..map.len() {
        for c in 0..map[r].len() {
            if map[r][c].to_string() == "*" {
                total += check(&map, &r, &c);
            }
        }
    }
    println!("{}", total);
}

fn check(map: &Vec<Vec<char>>, r: &usize, c: &usize) -> u32 {
    let mut nums: Vec<u32> = Vec::new();
    if c > &0 && map[*r][c - 1].is_digit(10) {
        nums.push(check_number(&map, &r, &(c - 1)));
    }
    if c < &(map[*r].len() - 1) && map[*r][c + 1].is_digit(10) {
        nums.push(check_number(&map, &r, &(c + 1)));
    }
    let left = max(&1, c) - 1;
    let right = min(c + 1, map[*r].len() - 1);
    let mut unum = false;
    let mut dnum = false;
    for c in left..=right {
        if r > &0 {
            if !unum && map[r - 1][c].is_digit(10) {
                nums.push(check_number(&map, &(r - 1), &c));
                unum = true;
            } else if !map[r - 1][c].is_digit(10) {
                unum = false;
            }
        }
        if r < &(map.len() - 1) {
            if !dnum && map[r + 1][c].is_digit(10) {
                nums.push(check_number(&map, &(r + 1), &c));
                dnum = true;
            } else if !map[r + 1][c].is_digit(10) {
                dnum = false;
            }
        }
    }
    if nums.len() == 2 {
        return nums[0] * nums[1];
    }
    0
}

fn check_number(map: &Vec<Vec<char>>, r: &usize, c: &usize) -> u32 {
    let mut buf: String = map[*r][*c].to_string();
    for i in (0..*c).rev() {
        if map[*r][i].is_digit(10) {
            buf = map[*r][i].to_string() + &buf;
        } else {
            break;
        }
    }
    for j in c + 1..map[*r].len() {
        if map[*r][j].is_digit(10) {
            buf = buf + &map[*r][j].to_string();
        } else {
            break;
        }
    }
    match buf.parse() {
        Err(_) => 0,
        Ok(n) => n,
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
