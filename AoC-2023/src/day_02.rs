use crate::Solution;
use regex::Regex;
pub struct Day02;

struct Counts {
    red: u8,
    green: u8,
    blue: u8,
}

fn update_counts(color: &str, count: &u8, current: &mut Counts) -> () {
    if color == "red" {
        current.red = std::cmp::max(*count, current.red);
    } else if color == "green" {
        current.green = std::cmp::max(*count, current.green);
    } else if color == "blue" {
        current.blue = std::cmp::max(*count, current.blue);
    } else {
        panic!()
    }
}

fn reveal_is_possible(color: &str, count: &u8) -> bool {
    if color == "red" {
        return *count <= 12;
    } else if color == "green" {
        return *count <= 13;
    } else if color == "blue" {
        return *count <= 14;
    } else {
        panic!()
    }
}

impl Solution for Day02 {

    type Item = u64;

    fn day(&self) -> u8 {
        return 2;
    }

    fn solve(&self, lines: &Vec<String>) -> (Result<Self::Item, &str>, Result<Self::Item, &str>) {
        let mut part1: Self::Item = 0;
        let mut part2: Self::Item = 0;
        let re = Regex::new(r"([0-9]+) (red|green|blue)").unwrap();
        for (game, line) in lines.iter().enumerate() {
            let mut game_possible: bool = true;
            let mut counts = Counts {
                red: 0,
                green: 0,
                blue: 0,
            };
            let parts = line.split(";");
            for part in parts {
                println!("{}", part);
                for caps in re.captures_iter(part) {
                    let cap: String = caps[0].to_string();
                    println!("{}", cap);
                    let subparts: Vec<&str> = cap.split(" ").collect();
                    let count: u8 = subparts[0].parse().unwrap();
                    let color: &str = subparts[1];
                    if !reveal_is_possible(color, &count) {
                        game_possible = false;
                    }
                    update_counts(color, &count, &mut counts);
                }
            }
            if game_possible {
                part1 += (game + 1) as Self::Item;
            }
            part2 += counts.red as Self::Item * counts.green as Self::Item * counts.blue as Self::Item;
        }
        return (Ok(part1), Ok(part2));
    }
    
}