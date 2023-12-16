use crate::Solution;
use regex::Regex;
use std::collections::HashMap;

pub struct Day01;

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

impl Solution for Day01 {
    type Item = u32;

    fn day(&self) -> u8 {
        return 1;
    }

    fn part1(&self, lines: &Vec<String>) -> Result<Self::Item, &str> { 
        let re = Regex::new(r"([0-9])").unwrap();
        let result = get_sum(&lines, re, None);
        return Ok(result);
    }
    
    fn part2(&self, lines: &Vec<String>) -> Result<Self::Item, &str> { 
        let re = Regex::new("([0-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
        let map = create_map();
        let result = get_sum(&lines, re, Some(&map));
        return Ok(result);
    }
    
}