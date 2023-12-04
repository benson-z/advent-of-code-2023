use std::cmp::max;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename = "input.txt";
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
            let l: Vec<&str> = line.split(": ").collect();
            let sets: Vec<&str> = l[1].split(" | ").collect();
            let win: Vec<&str> = sets[0].split_whitespace().collect();
            let have: Vec<&str> = sets[1].split_whitespace().collect();
            let mut points: u32 = 0;
            for card in have {
                if win.contains(&card) {
                    points = max(points + 1, points * 2);
                }
            }
            println!("points: {}", points);
            total += points;
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
