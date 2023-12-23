
use crate::Solution;

pub struct Day05;

#[derive(Copy, Clone)]
struct Range {
    dest_start: u64,
    source_start: u64,
    len: u64
}

type Map = Vec<Range>;

fn get_seeds_part1(seeds_row: &String) -> Vec<u64> {
    let seeds:Vec<u64> = seeds_row[6..].split(' ').
    filter(|s| !s.is_empty()).
    map(|s| s.parse().unwrap()) .collect();
    return seeds;
}

fn get_seeds_part2(seeds_row: &String) -> Vec<Range> {
    let mut seeds: Vec<Range> = Vec::new();
    let nums: Vec<u64> = seeds_row[6..].split(' ').
    filter(|s| !s.is_empty()).
    map(|s| s.parse().unwrap()) .collect();

    for i in (0..nums.len()-1).step_by(2) {
        let seed_range = Range {
            dest_start: 0,
            source_start: nums[i],
            len: nums[i+1]
        };
        seeds.push(seed_range);
    }
    return seeds;
}

fn get_maps(input: &Vec<String>) -> Vec<Map>
{
    let mut map: Map = Vec::new(); 
    let mut maps: Vec<Map> = Vec::new();
    for line in input.iter() {
        if line.is_empty() {
            if map.len() > 0 {
                maps.push(map);
            }
            map = Map::new();
        }
        else if line.chars().next().unwrap().is_numeric() {
            let nums: Vec<u64> = line.split(' ').map(|s| s.parse().unwrap()).collect();
            let range = Range{
                dest_start: nums[0],
                source_start: nums[1],
                len: nums[2]
            };
            map.push(range);
        }
    }
    maps.push(map);
    return maps;
}

fn propagate(mut seed: u64, maps: &Vec<Map>, mut map_index: u8) -> u64 {
    let mut res = seed;
    if map_index < maps.len() as u8 { 
           for range in &maps[map_index as usize] {
            if seed >= range.source_start && seed < (range.source_start + range.len) {
                res = range.dest_start + (seed - range.source_start);
                break;
            }
        }
        map_index += 1;
        seed = propagate(res, maps, map_index);
    }
    return seed;
}

fn map_ranges(input: &Vec<Range>, map_range: &Vec<Range>) -> Vec<Range> {
    let mut start: Vec<Range> = input.to_owned();
    let mut output: Vec<Range> = Vec::new();
    let mut rem: Vec<Range> = Vec::new();
    
    for map_rng in map_range.iter() {
        rem = Vec::new();
        while start.len() > 0 {
            let rng = start.pop().unwrap();
            let x1 = rng.source_start;
            let x2 = rng.source_start + rng.len-1;
            let y1 = map_rng.source_start;
            let y2 = map_rng.source_start + map_rng.len-1;
            if (x2 < y1) || (x1 > y2) {
                // no overlap
                rem.push(rng);
            } else {
                let mut lower = x1;
                let mut upper = x2;
                if x1 < y1 {
                    rem.push(
                        Range {
                            dest_start: 0,
                            source_start: x1,
                            len: y1 - x1
                        }
                    );
                    lower = y1;
                }
                if x2 > y2 {
                    rem.push(
                        Range {
                            dest_start: 0,
                            source_start: y2+1,
                            len: x2-y2
                        }
                    );
                    upper = y2;
                }
                output.push(
                    Range {
                        dest_start: 0,
                        source_start: lower + map_rng.dest_start - map_rng.source_start,
                        len: upper - lower + 1
                    }
                );
            }
        }
        start = rem.to_owned();
    } 
    output.append(&mut rem);
    return output;
    }

fn get_min(ranges: &Vec<Range>) -> u64 {
    let mut res: u64 = 0;
    for range in ranges {
        if res == 0 || range.source_start < res {
            res = range.source_start;
        }
    }
    return res;
}


impl Solution for Day05 {

    type Item = u64;

    fn day(&self) -> u8 {
        return 5;
    }

    fn part1(&self, input: &Vec<String>) -> Result<Self::Item, &str> {
        let seeds = get_seeds_part1(&input[0]);
        let maps = get_maps(input);
        let mut locs: Vec<u64> = Vec::new();
        for seed in seeds.iter() {
            let loc = propagate(*seed, &maps, 0);
            locs.push(loc);
        }
        return Ok(*locs.iter().min().unwrap());
    }

    fn part2(&self, input: &Vec<String>) -> Result<Self::Item, &str> {
        let seed_ranges = get_seeds_part2(&input[0]);
        let maps = get_maps(input);

        let mut ranges = seed_ranges;
        for map in maps {
            ranges = map_ranges(&ranges, &map);
        }
        let res = get_min(&ranges);
        return Ok(res);
    }

}



