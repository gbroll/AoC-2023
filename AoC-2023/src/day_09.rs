use crate::Solution;

pub struct Day09;

enum Direction {
    Forward,
    Backward
}

impl Day09 {
    fn parse(input: &Vec<String>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        for line in input {
            res.push(line.split(' ').map(|s| s.parse().unwrap()).collect());
        }
        return res;
    }

    fn get_next(&self, row: &Vec<i32>, dir: Direction) -> i32 {
        if row.len() == 0 || row.iter().all(|&x| x == 0) {
            return 0;
        } else {
            let diff: Vec<i32> = row.windows(2).map(|s| s[1] - s[0]).collect();
            match dir {
                Direction::Forward => {
                    return row.last().unwrap() + self.get_next(&diff, dir);
                }
                Direction::Backward => {
                    return row.first().unwrap() - self.get_next(&diff, dir);
                }
            }
        }
    }
}

impl Solution for Day09 {
    fn day(&self) -> u8 {
        return 9;
    }

    type Item = i32;

    fn solve(&self, input: &Vec<String>) -> (Result<Self::Item, &str>, Result<Self::Item, &str>) {
        let data = Day09::parse(input);
        let mut part1: Self::Item = 0; 
        let mut part2: Self::Item = 0;
        for row in data {
            part1 += Day09::get_next(&self, &row, Direction::Forward);
            part2 += Day09::get_next(&self, &row, Direction::Backward)
        }
        return (Ok(part1), Ok(part2));
    }
}