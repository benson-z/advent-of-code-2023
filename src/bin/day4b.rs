use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename = "input.txt";
    let mut cards: Vec<u32> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        let lines: Vec<String> = lines.filter_map(Result::ok).collect();
        // loop through lines
        for line in lines.iter().rev() {
            let l: Vec<&str> = line.split(": ").collect();
            let sets: Vec<&str> = l[1].split(" | ").collect();
            let win: Vec<&str> = sets[0].split_whitespace().collect();
            let have: Vec<&str> = sets[1].split_whitespace().collect();
            let mut matches: u32 = 0;
            for card in have {
                if win.contains(&card) {
                    matches += 1;
                }
            }
            let mut copies: u32 = 1;
            for i in 0..matches {
                copies += cards[cards.len() - i as usize - 1];
            }
            cards.push(copies);
        }
    }
    let total: u32 = cards.iter().sum();
    println!("{}", total);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
