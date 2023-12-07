use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CardType {
    FiveOfAKind = 1,
    FourOfAKind = 2,
    FullHouse = 3,
    ThreeOfAKind = 4,
    TwoPair = 5,
    OnePair = 6,
    HighCard = 7,
    Unknown = -1,
}

pub fn find_type(hand: &str) -> CardType {
    let res = hand.chars().fold(HashMap::new(), |mut acc, char| {
        acc.insert(char, acc.get(&char).unwrap_or(&0) + 1);
        acc
    });

    let joker_count = *res.get(&'J').unwrap_or(&0);

    match res.len() {
        1 => CardType::FiveOfAKind,
        2 => {
            if res.values().any(|&v| v == 1) {
                match joker_count {
                    1 => CardType::FiveOfAKind,
                    4 => CardType::FiveOfAKind,
                    _ => CardType::FourOfAKind,
                }
            } else {
                match joker_count {
                    2 => CardType::FiveOfAKind,
                    3 => CardType::FiveOfAKind,
                    _ => CardType::FullHouse,
                }
            }
        }
        3 => {
            if res.values().any(|&v| v == 3) {
                match joker_count {
                    1 => CardType::FourOfAKind,
                    3 => CardType::FourOfAKind,
                    _ => CardType::ThreeOfAKind,
                }
            } else {
                match joker_count {
                    1 => CardType::FullHouse,
                    2 => CardType::FourOfAKind,
                    _ => CardType::TwoPair,
                }
            }
        }
        4 => match joker_count {
            1 => CardType::ThreeOfAKind,
            2 => CardType::ThreeOfAKind,
            _ => CardType::OnePair,
        },
        5 => match joker_count {
            1 => CardType::OnePair,
            2 => CardType::ThreeOfAKind,
            3 => CardType::FourOfAKind,
            4 => CardType::FiveOfAKind,
            _ => CardType::HighCard,
        },
        _ => CardType::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_type_five() {
        assert_eq!(CardType::FiveOfAKind, find_type("AAAAA"));
        assert_eq!(CardType::FiveOfAKind, find_type("JJJJJ"));
    }

    #[test]
    fn test_find_type_four() {
        assert_eq!(CardType::FourOfAKind, find_type("AAAAB"));
        assert_eq!(CardType::FiveOfAKind, find_type("AAAAJ"));
        assert_eq!(CardType::FiveOfAKind, find_type("JJJJA"));
    }

    #[test]
    fn test_find_type_full_house() {
        assert_eq!(CardType::FullHouse, find_type("AAABB"));
        assert_eq!(CardType::FiveOfAKind, find_type("AAAJJ"));
        assert_eq!(CardType::FiveOfAKind, find_type("AAJJJ"));
    }

    #[test]
    fn test_find_type_three() {
        assert_eq!(CardType::ThreeOfAKind, find_type("AAABC"));
        assert_eq!(CardType::FourOfAKind, find_type("AAABJ"));
        assert_eq!(CardType::FourOfAKind, find_type("JJJBA"));
    }

    #[test]
    fn test_find_type_two_pair() {
        assert_eq!(CardType::TwoPair, find_type("AABBC"));
        assert_eq!(CardType::FullHouse, find_type("AABBJ"));
        assert_eq!(CardType::FourOfAKind, find_type("JJBBA"));
    }

    #[test]
    fn test_find_type_one_pair() {
        assert_eq!(CardType::OnePair, find_type("AABCD"));
        assert_eq!(CardType::ThreeOfAKind, find_type("AABCJ"));
        assert_eq!(CardType::ThreeOfAKind, find_type("JJABC"));
    }

    #[test]
    fn test_find_type_high_card() {
        assert_eq!(CardType::HighCard, find_type("ABCDE"));
        assert_eq!(CardType::OnePair, find_type("ABCDJ"));
    }
}
