use AoC_2023::*;
use AoC_2023::utils;

fn run(day: u8) {
    let lines = utils::get_input(day);
    match day {
        1 => day_01::Day01.run(&lines),
        2 => day_02::Day02.run(&lines),
        3 => day_03::Day03.run(&lines),
        4 => day_04::Day04.run(&lines),
        5 => day_05::Day05.run(&lines),
        6 => day_06::Day06.run(&lines),
        7 => day_07::Day07.run(&lines),
        8 => day_08::Day08.run(&lines),
        _ => panic!("Unexpected day"),
    };
}

fn main() {
    let day: u8 = 8;
    run(day);
}
