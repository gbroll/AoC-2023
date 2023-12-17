use crate::Solution;

pub struct Day04;

fn update_counts(counts: &mut Vec<u32>, card_num: usize, num_matches: u8) {
    for i in 0..num_matches {
        let index = 1 + card_num + i as usize;
        if index >= counts.len() {
            counts.push(counts[card_num]);
        } else {
            counts[index] += counts[card_num]; 
        }
    }
}

impl Solution for Day04 {
    type Item = u32;

    fn day(&self) -> u8 {
        return 4;
    }

    fn solve(&self, input: &Vec<String>) -> (Result<Self::Item, &str>, Result<Self::Item, &str>) {
        let mut part1: Self::Item = 0;
        let mut card_counts: Vec<Self::Item> = Vec::new();
        for (line_num, line) in input.iter().enumerate() {
            if line_num >= card_counts.len() {
                card_counts.push(1);
            } else {
                card_counts[line_num] += 1;
            }

            let mut num_matches: u8 = 0;
            let mut winners: Vec<u16> = Vec::new();

            let parts: Vec<&str> = line.split('|').collect();
            let winners_str: &str = &parts[0].split(':').last().unwrap().trim();
            let cards_str = &parts[1].trim();
            for winner in winners_str.split(' ').filter(|s| !s.is_empty()) {
                winners.push(winner.parse::<u16>().unwrap());
            }
            for card in cards_str.split(' ').filter(|s| !s.is_empty()) {
                let my_card = card.parse::<u16>().unwrap();
                if winners.contains(&my_card) {
                    num_matches += 1;

                }
            }
            let game_score = match num_matches {
                0 => 0,
                _ => (2 as u32).pow((num_matches as u32)-1)
            };
            part1 += game_score as Self::Item;
            update_counts(&mut card_counts, line_num, num_matches);
        }
        let part2: Self::Item = card_counts.iter().sum();
        return (Ok(part1), Ok(part2));
    }

}

