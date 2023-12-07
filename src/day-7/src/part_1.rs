use std::collections::HashMap;

const CARDS: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

#[derive(Debug, Clone, Copy)]
enum CardType {
    FiveOfAKind = 1,
    FourOfAKind = 2,
    FullHouse = 3,
    ThreeOfAKind = 4,
    TwoPair = 5,
    OnePair = 6,
    HighCard = 7,
    Unknown = -1,
}

fn find_type(hand: &str) -> CardType {
    let res = hand.chars().fold(HashMap::new(), |mut acc, char| {
        acc.insert(char, acc.get(&char).unwrap_or(&0) + 1);
        acc
    });

    match res.len() {
        1 => CardType::FiveOfAKind,
        2 => {
            if res.values().any(|&v| v == 1) {
                CardType::FourOfAKind
            } else {
                CardType::FullHouse
            }
        }
        3 => {
            if res.values().any(|&v| v == 3) {
                CardType::ThreeOfAKind
            } else {
                CardType::TwoPair
            }
        }
        4 => CardType::OnePair,
        5 => CardType::HighCard,
        _ => CardType::Unknown,
    }
}

pub fn solve(lines: &Vec<String>) -> u64 {
    let mut cards = lines
        .iter()
        .map(|line| {
            let pair = line.split_whitespace().collect::<Vec<_>>();
            let hand = pair[0];
            let bid = pair[1];
            let card_type = find_type(hand);
            (card_type, hand, bid)
        })
        .collect::<Vec<_>>();

    cards.sort_by(|a, b| {
        let card_type_a = a.0 as u32;
        let card_type_b = b.0 as u32;

        let chars_a = a.1.chars().collect::<Vec<_>>();
        let chars_b = b.1.chars().collect::<Vec<_>>();

        card_type_b
            .cmp(&card_type_a)
            .then_with(|| {
                let a = CARDS.iter().position(|&c| c == chars_a[0]).unwrap();
                let b = CARDS.iter().position(|&c| c == chars_b[0]).unwrap();

                b.cmp(&a)
            })
            .then_with(|| {
                let a = CARDS.iter().position(|&c| c == chars_a[1]).unwrap();
                let b = CARDS.iter().position(|&c| c == chars_b[1]).unwrap();

                b.cmp(&a)
            })
            .then_with(|| {
                let a = CARDS.iter().position(|&c| c == chars_a[2]).unwrap();
                let b = CARDS.iter().position(|&c| c == chars_b[2]).unwrap();

                b.cmp(&a)
            })
            .then_with(|| {
                let a = CARDS.iter().position(|&c| c == chars_a[3]).unwrap();
                let b = CARDS.iter().position(|&c| c == chars_b[3]).unwrap();

                b.cmp(&a)
            })
            .then_with(|| {
                let a = CARDS.iter().position(|&c| c == chars_a[4]).unwrap();
                let b = CARDS.iter().position(|&c| c == chars_b[4]).unwrap();

                b.cmp(&a)
            })
    });

    cards
        .iter()
        .fold((0, 1), |(acc, index), card| {
            let bid = card.2.parse::<u64>().unwrap();
            (acc + (bid * index), index + 1)
        })
        .0
}
