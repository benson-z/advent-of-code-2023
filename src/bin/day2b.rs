use std::cmp::max;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename = "input.txt";
    let mut total = 0;

    if let Ok(lines) = read_lines(filename) {
        // loop through lines
        for line in lines {
            let line = match line {
                Ok(l) => l,
                Err(_) => {
                    continue;
                }
            };

            // split game id from game data
            let parts: Vec<&str> = line.split(": ").collect();

            // split game data into individual bags
            let bags: Vec<&str> = parts[1].split("; ").collect();

            // maximum dice needed for each game
            let mut maxr = 0;
            let mut maxb = 0;
            let mut maxg = 0;

            for bag in bags {
                let draws: Vec<&str> = bag.split(", ").collect();

                // counters for each color
                let mut r = 0;
                let mut g = 0;
                let mut b = 0;
                for draw in draws {
                    let components: Vec<&str> = draw.split(" ").collect();
                    let number: u32 = components[0].parse().expect("Not a num");

                    if &components[1] == &"red" {
                        r += number;
                    } else if &components[1] == &"blue" {
                        b += number;
                    } else if &components[1] == &"green" {
                        g += number;
                    }
                }

                // update the maximum dice values
                maxg = max(maxg, g);
                maxr = max(maxr, r);
                maxb = max(maxb, b);
            }

            // update total
            total += maxg * maxb * maxr;
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
