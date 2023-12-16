use std::fmt::Display;

pub mod utils;
pub mod day_01;
pub mod day_02;
pub mod day_03;


pub trait Solution {
    type Item: Display;
    
    fn day(&self) -> u8;

    fn part1(&self, input: &Vec<String>) -> Result<Self::Item, &str> {
        return Err("Not yet implemented");
    }

    fn part2(&self, input: &Vec<String>) -> Result<Self::Item, &str> {
        return Err("Not yet implemented");
    }

    fn solve(&self, input: &Vec<String>) -> (Result<Self::Item, &str>, Result<Self::Item, &str>) {
        return (self.part1(input), self.part2(input));
    }

    fn print_res(&self, day: u8, res: (Result<Self::Item, &str>, Result<Self::Item, &str>)) {
            let parts = vec![res.0, res.1];
            for (part, part_res) in parts.iter().enumerate() {
                match part_res {
                    Ok(val) => {
                        println!("**AoC 2023 day {} part {}: {} **", 
                        day.to_string(), part.to_string(), val.to_string()); 
                    }
                    Err(msg) => {
                        println!("**AoC 2023 day {} part {}: {} **", 
                        day.to_string(), part.to_string(), msg.to_string());
                    }
                }
            }
    }

    fn run(&self, input: &Vec<String>) {
        let res = self.solve(input);
        self.print_res(self.day(), res);
    }
}