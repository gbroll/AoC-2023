use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn create_map() -> HashMap<String, u8> {
    let mut map: HashMap<String, u8> = HashMap::new();
    map.insert(String::from("one"), 1);
    map.insert(String::from("two"), 2);
    map.insert(String::from("three"), 3);
    map.insert(String::from("four"), 4);
    map.insert(String::from("five"), 5);
    map.insert(String::from("six"), 6);
    map.insert(String::from("seven"), 7);
    map.insert(String::from("eight"), 8);
    map.insert(String::from("nine"), 9);
    return map;
}

fn get_sum(lines: &Vec<String>, re: Regex, map: Option<&HashMap<String, u8>>) -> u32 {
    let mut sum: u32 = 0;
    for line in lines.iter() {
        let mut first: u8 = 0;
        let mut last: u8 = 0;
        let mut first_matched: bool = false;

        let mut i = 0;
        while i < line.len() {
            let substr = &line[i..];
            i += 1;
            for caps in re.captures_iter(substr) {
                let mut num_str = "".to_string();
                num_str.push_str(&caps[1].to_string());
                if num_str.len() > 1 {
                    last = Option::expect(map, "err").get(&num_str).unwrap().to_owned();
                } else {
                    last = num_str.parse().unwrap();
                }
                if !first_matched {
                    first = last;
                    first_matched = true;
                }
            }
        }
        let mut num_str = "".to_string();
        num_str.push_str(&first.to_string());
        num_str.push_str(&last.to_string());
        let num: u32 = num_str.parse().unwrap();
        sum += num;
    }
    return sum;
}

fn day_01() -> (u32, u32) {

    let lines = read_lines("input/day_01.txt");

    let mut re = Regex::new(r"([0-9])").unwrap();
    let part1 = get_sum(&lines, re, None);

    re = Regex::new("([0-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let map = create_map();
    let part2 = get_sum(&lines, re, Some(&map));

    return (part1, part2);
}

fn main() {
    let res: (u32, u32) = day_01();
    println!("**AoC-2023 day 1 part 1: {} **", res.0);
    println!("**AoC-2023 day 1 part 2: {} **", res.1);
}
