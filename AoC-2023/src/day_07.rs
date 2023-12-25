use crate::Solution;

pub struct Day07;

const CARDS: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

#[derive(PartialEq, Eq, Ord, PartialOrd)]
enum HandType {
    HighCard = 1,
    Pair,
    TwoPair,
    ThreeOaKind,
    FullHouse,
    FourOaKind,
    FiveOaKind,
}

#[derive(PartialEq, Eq, Ord)]
struct Hand {
    cards: [u8; 5],
    htype: HandType,
    bet: u32,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(
            self.htype.cmp(&other.htype).then(
                self.cards[0].cmp(&other.cards[0]).then(
                    self.cards[1].cmp(&other.cards[1]).then(
                        self.cards[2].cmp(&other.cards[2]).then(
                            self.cards[3]
                                .cmp(&other.cards[3])
                                .then(self.cards[4].cmp(&other.cards[4])),
                        ),
                    ),
                ),
            ),
        );
    }
}

fn get_handtype(card_counts_in: &Vec<u8>, with_jokers: bool) -> HandType {
    let mut card_counts= card_counts_in.to_owned();
    if with_jokers {
        let j_index = CARDS.iter().position(|&c| c == 'J').unwrap();
        let num_jokers = card_counts.remove(j_index);
        card_counts.sort();
        *card_counts.last_mut().unwrap() += num_jokers;
    }
    else {
        card_counts.sort();
    }

    let hand_type: HandType;
    match card_counts.pop().unwrap() {
        5 => hand_type = HandType::FiveOaKind,
        4 => hand_type = HandType::FourOaKind,
        3 => match card_counts.pop().unwrap() {
            2 => hand_type = HandType::FullHouse,
            _ => hand_type = HandType::ThreeOaKind,
        },
        2 => match card_counts.pop().unwrap() {
            2 => hand_type = HandType::TwoPair,
            _ => hand_type = HandType::Pair,
        },
        _ => hand_type = HandType::HighCard,
    }
    return hand_type;
}

fn make_hands(input: &Vec<String>, with_jokers: bool) -> Vec<Hand> {
    let mut hands: Vec<Hand> = Vec::new();
    for input_str in input {
        let parts: (&str, &str) = input_str.split_once(" ").unwrap();
        let mut cards = [0, 0, 0, 0, 0];
        let mut card_counts: Vec<u8> = vec![0; CARDS.len()];
        for (i, card) in parts.0.chars().enumerate() {
            card_counts[CARDS.iter().position(|&c| c == card).unwrap() as usize] += 1;
            if with_jokers && card == 'J' {
                    cards[i] = 1;
                } else {
                    cards[i] = CARDS.iter().position(|&c| c == card).unwrap() as u8 + 2;
                }
        }

        hands.push(Hand {
            cards: cards,
            htype: get_handtype(&card_counts, with_jokers),
            bet: parts.1.parse().unwrap(),
        });
    }
    return hands;
}

fn winnings(sorted_hands: &Vec<Hand>) -> u32 {
    let mut res: u32 = 0;
    for (i, hand) in sorted_hands.iter().enumerate() {
        res += (i+1) as u32 * hand.bet;
    }
    return res;
}

impl Solution for Day07 {
    fn day(&self) -> u8 {
        return 7;
    }

    type Item = u32;

    fn part1(&self, input: &Vec<String>) -> Result<Self::Item, &str> {
        let mut hands = make_hands(input, false);
        hands.sort();
        return Ok(winnings(&hands));
    }
    fn part2(&self, input: &Vec<String>) -> Result<Self::Item, &str> {
        let mut hands = make_hands(input, true);
        hands.sort();
        return Ok(winnings(&hands));
    }
}
