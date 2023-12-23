use crate::Solution;

pub struct Day06;

struct Race {
    time: u64,
    distance: u64,
}

fn get_races_part1(input: &Vec<String>) -> Vec<Race> {
    let mut races: Vec<Race> = Vec::new();
    let time_vec: Vec<&str> = input[0]
        .split(" ")
        .filter(|s| s.parse::<u64>().is_ok())
        .collect();
    let dist: Vec<&str> = input[1]
        .split(" ")
        .filter(|s| s.parse::<u64>().is_ok())
        .collect();
    for (i, t) in time_vec.iter().enumerate() {
        races.push(Race {
            time: t.parse().unwrap(),
            distance: dist[i].parse().unwrap(),
        });
    }
    return races;
}

fn get_races_part2(input: &Vec<String>) -> Vec<Race> {
    let time = input[0]
        .split(" ")
        .filter(|s| s.parse::<u64>().is_ok())
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()
        .unwrap();
    let dist = input[1]
        .split(" ")
        .filter(|s| s.parse::<u64>().is_ok())
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()
        .unwrap();
    return vec![Race {
        time: time,
        distance: dist,
    }];
}

fn eval_races(races: &Vec<Race>) -> u64 {
    let mut result: u64 = 1;
    for race in races {
        let t: f64 = race.time as f64;
        let d: f64 = race.distance as f64;
        let mut mn: u64 = (t / 2.0 - f64::sqrt((t / 2.0) * (t / 2.0) - d)).floor() as u64;
        let mut mx: u64 = (t / 2.0 + f64::sqrt((t / 2.0) * (t / 2.0) - d)).ceil() as u64;

        if mn * (race.time - mn) <= race.distance {
            mn += 1;
        }
        if mx * (race.time - mx) <= race.distance {
            mx -= 1;
        }

        println!("{},{}, {}", mn, mx, (mx - mn) as u64 + 1);
        result *= (mx - mn) as u64 + 1;
    }
    return result;
}

impl Solution for Day06 {
    type Item = u64;

    fn day(&self) -> u8 {
        return 6;
    }

    fn part1(&self, input: &Vec<String>) -> Result<Self::Item, &str> {
        let races = get_races_part1(input);
        let res = eval_races(&races);
        return Ok(res);
    }

    fn part2(&self, input: &Vec<String>) -> Result<Self::Item, &str> {
        let races = get_races_part2(input);
        let res = eval_races(&races);
        return Ok(res);
    }
}
