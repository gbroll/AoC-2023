use crate::Solution;
use std::collections::HashMap;
use num::integer::lcm;

struct Node {
    left: String,
    right: String,
}

fn parse(input: &Vec<String>) -> (Vec<bool>, HashMap<String, Node>) {
    let mut moves: Vec<bool> = Vec::new();
    let mut map: HashMap<String, Node> = HashMap::new();

    moves = input[0].chars().map(|c| c == 'L').collect();
    for line in &input[2..] {
        map.insert(
            line[0..3].to_string(),
            Node {
                left: line[7..10].to_string(),
                right: line[12..15].to_string(),
            },
        );
    }
    return (moves, map);
}

fn get_start_positions(map: &HashMap<String, Node>) -> Vec<String> {
    let res: Vec<String> = map
        .into_iter()
        .filter_map(|(key, _)| {
            if key.ends_with("A") {
                Some(key.to_owned())
            } else {
                None
            }
        })
        .collect();
    return res;
}

fn get_cycle_length(mut pos: String, moves: &Vec<bool>, map: &HashMap<String, Node>) -> u64 {
    let mut num_moves: u64= 0;
    let mut i: usize = 0;
    loop {
        let mv = moves[i];
        num_moves += 1;
        i += 1;
        if i >= moves.len() {
            i = 0;
        }
        if mv {
            pos = map.get(&pos).unwrap().left.to_owned();
        } else {
            pos = map.get(&pos).unwrap().right.to_owned();
        }
        if pos.ends_with("Z") {
            break;
        }
    }
    return num_moves;
}

impl Solution for Day08 {
    fn day(&self) -> u8 {
        return 8;
    }

    type Item = u64;

    fn part1(&self, input: &Vec<String>) -> Result<Self::Item, &str> {
        let (moves, map) = parse(input);
        let mut pos = "AAA".to_string();
        let mut num_moves: Self::Item = 0;
        let mut i: usize = 0;
        loop {
            let mv = moves[i];
            num_moves += 1;
            i += 1;
            if i >= moves.len() {
                i = 0;
            }
            if mv {
                pos = map.get(&pos).unwrap().left.to_owned();
            } else {
                pos = map.get(&pos).unwrap().right.to_owned();
            }
            if pos == "ZZZ" {
                break;
            }
        }
        return Ok(num_moves);
    }

    fn part2(&self, input: &Vec<String>) -> Result<Self::Item, &str> {
        let (moves, map) = parse(input);
        let start_positions: Vec<String> = get_start_positions(&map);
        let mut cycle_lengths: Vec<u64> = Vec::new();
        for start_pos in start_positions {
            cycle_lengths.push(get_cycle_length(start_pos, &moves, &map))
        }
        let res = cycle_lengths.into_iter().reduce(lcm).unwrap();
        return Ok(res);
    }
}

pub struct Day08;
