use regex::Regex;
use std::fs::read_to_string;

struct Counts {
    red: u8,
    green: u8,
    blue: u8,
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
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

fn solve(lines: &Vec<String>, re: Regex) -> (u32, u32) {
    let mut part1: u32 = 0;
    let mut part2: u32 = 0;
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
            part1 += (game + 1) as u32;
        }
        part2 += counts.red as u32 * counts.green as u32 * counts.blue as u32;
    }
    return (part1, part2);
}

fn main() {
    let lines = read_lines("input/day_02.txt");
    let re = Regex::new(r"([0-9]+) (red|green|blue)").unwrap();
    let (part1, part2) = solve(&lines, re);
    println!("**AoC-2023 day 2 part 1: {} **", part1);
    println!("**AoC-2023 day 2 part 1: {} **", part2);
}
