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
        let mut buffer: String = String::new();
        for c in 0..map[r].len() {
            if map[r][c].is_digit(10) {
                buffer += &map[r][c].to_string();
            } else {
                if c > 0 && !buffer.is_empty() && check(&map, c - buffer.len(), c - 1, &r) {
                    let part_num: u32 = buffer.parse().expect("not a number");
                    total += part_num;
                }
                buffer = String::new();
            }
        }
        if !buffer.is_empty() && check(&map, map[r].len() - buffer.len(), map[r].len() - 1, &r) {
            let part_num: u32 = buffer.parse().expect("not a number");
            total += part_num;
        }
    }
    println!("{}", total);
}

fn check(map: &Vec<Vec<char>>, startx: usize, endx: usize, y: &usize) -> bool {
    if startx > 0 && map[*y][startx - 1].to_string() != "." {
        return true;
    }
    if endx < (map[*y].len() - 1) && map[*y][endx + 1].to_string() != "." {
        return true;
    }
    let left = max(1, startx) - 1;
    let right = min(endx + 1, map[*y].len() - 1);
    for c in left..=right {
        if y > &0 && is_symbol(&map[y - 1][c]) {
            return true;
        }
        if y < &(map.len() - 1) && is_symbol(&map[y + 1][c]) {
            return true;
        }
    }
    false
}

fn is_symbol(c: &char) -> bool {
    !(c.is_digit(10) || c.to_string() == ".")
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
