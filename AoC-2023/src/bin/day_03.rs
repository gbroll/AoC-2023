use std::collections::HashMap;
use std::fs::read_to_string;

struct XY {
    x: i16,
    y: i16,
}

fn get_neighbours(pos: &XY, bounds: XY) -> Vec<XY> {
    let mut res: Vec<XY> = Vec::new();
    for x in vec![-1, 0, 1] {
        for y in vec![-1, 0, 1] {
            if (0..bounds.x).contains(&(pos.x + x)) && (0..bounds.y).contains(&(pos.y + y)) {
                if x != 0 || y != 0 {
                    res.push(XY {
                        x: pos.x + x,
                        y: pos.y + y,
                    });
                }
            }
        }
    }
    return res;
}

fn get_value_at_pos(pos: &XY, grid: &Vec<Vec<char>>) -> char {
    return grid[pos.y as usize][pos.x as usize];
}

fn has_adjacent_symbol(pos: &XY, grid: &Vec<Vec<char>>) -> bool {
    let bounds = XY {
        x: grid[0].len() as i16,
        y: grid.len() as i16,
    };
    let neigbours = get_neighbours(pos, bounds);
    for nbr in neigbours.iter() {
        let val = get_value_at_pos(nbr, grid);
        if !val.is_alphanumeric() && val != '.' {
            return true;
        }
    }
    return false;
}

fn sum_gear_ratios(gears: &HashMap<u32, Vec<u32>>) -> u32 {
    let mut res: u32 = 0;
    for (_, val) in gears.iter() {
        if val.len() == 2 {
            res += (val[0] * val[1]);
        }
    }
    return res;
}

fn get_adjacent_gear(pos: &XY, grid: &Vec<Vec<char>>) -> Option<XY> {
    let bounds = XY {
        x: grid[0].len() as i16,
        y: grid.len() as i16,
    };
    let neigbours = get_neighbours(pos, bounds);
    for nbr in neigbours.iter() {
        let val = get_value_at_pos(nbr, grid);
        if val == '*' {
            return Some(XY { x: nbr.x, y: nbr.y });
        }
    }
    return None;
}

fn read_lines(filename: &str) -> Vec<Vec<char>> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        let chars = line.chars().collect();
        result.push(chars);
    }
    result
}

fn solve(chars: &Vec<Vec<char>>) -> (u32, u32) {
    let mut gears: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut val_acc = String::new();
    let mut is_part_num: bool = false;
    let mut gear: Option<XY> = None;
    let mut part1: u32 = 0;
    for (y, row) in chars.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            let pos = XY {
                x: x as i16,
                y: y as i16,
            };
            println!("x: {}, y: {}, val: {}", x, y, val);
            if val.is_numeric() {
                val_acc.push(*val);
                if has_adjacent_symbol(&pos, chars) {
                    is_part_num = true;
                }
                let nbr_gear = get_adjacent_gear(&pos, chars);
                if gear.is_none() && nbr_gear.is_some() {
                    gear = nbr_gear;
                }
            } else {
                if !val_acc.is_empty() {
                    if is_part_num {
                        let part_num: u32 = val_acc.parse().unwrap();
                        part1 += part_num;

                        if gear.is_some() {
                            let g = gear.unwrap();
                            let id: u32 = g.x as u32 * 1000 + g.y as u32; // ugly hack but should work
                            if gears.contains_key(&id) {
                                gears.get_mut(&id).unwrap().push(part_num);
                            } else {
                                gears.insert(id, vec![part_num]);
                            }
                        }
                    }
                }
                val_acc.clear();
                is_part_num = false;
                gear = None;
            }
        }
    }
    let part2: u32 = sum_gear_ratios(&gears);
    return (part1, part2);
}

fn main() {
    let grid = read_lines("input/day_03.txt");
    let (part1, part2) = solve(&grid);
    println!("**AoC-2023 day 3 part 1: {} **", part1);
    println!("**AoC-2023 day 3 part 2: {} **", part2);
}
