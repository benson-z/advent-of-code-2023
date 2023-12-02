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

            // convert game_id into u32
            let game_id: u32 = parts[0]
                .strip_prefix("Game ")
                .expect("parsing error")
                .parse()
                .expect("not a number");

            // split game data into individual bags
            let bags: Vec<&str> = parts[1].split("; ").collect();

            // store whether the game has failed
            let mut failed = false;

            for bag in bags {
                let draws: Vec<&str> = bag.split(", ").collect();

                // counters for each color
                let mut r = 0;
                let mut g = 0;
                let mut b = 0;

                for draw in draws {
                    let components: Vec<&str> = draw.split(" ").collect();
                    let number: u32 = components[0].parse().expect("not a number");

                    if &components[1] == &"red" {
                        r += number;
                    } else if &components[1] == &"blue" {
                        b += number;
                    } else if &components[1] == &"green" {
                        g += number;
                    }
                }

                // check if the game has failed
                if r > 12 || g > 13 || b > 14 {
                    failed = true;
                    break;
                }
            }
            if !failed {
                total += game_id;
            }
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
