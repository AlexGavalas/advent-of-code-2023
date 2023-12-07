use crate::find_type_2::find_type;

const CARDS: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

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
