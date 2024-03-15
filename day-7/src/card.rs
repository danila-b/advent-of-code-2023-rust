use std::{collections::HashMap, hash::Hash};

/// Cards Enum represent card type along with a relative power of a card
/// for comparison with other cards
#[derive(Debug)]
enum CardType {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

#[derive(Debug)]
pub struct Card {
    card_type: CardType,
    pub rank: u8,
}

impl Card {
    pub fn from_char(char: &char) -> Result<Self, String> {
        match char {
            '1' => Ok(Card {
                card_type: CardType::One,
                rank: 1,
            }),
            '2' => Ok(Card {
                card_type: CardType::Two,
                rank: 2,
            }),
            '3' => Ok(Card {
                card_type: CardType::Three,
                rank: 3,
            }),
            '4' => Ok(Card {
                card_type: CardType::Four,
                rank: 4,
            }),
            '5' => Ok(Card {
                card_type: CardType::Five,
                rank: 5,
            }),
            '6' => Ok(Card {
                card_type: CardType::Six,
                rank: 6,
            }),
            '7' => Ok(Card {
                card_type: CardType::Seven,
                rank: 7,
            }),
            '8' => Ok(Card {
                card_type: CardType::Eight,
                rank: 8,
            }),
            '9' => Ok(Card {
                card_type: CardType::Nine,
                rank: 9,
            }),
            'T' => Ok(Card {
                card_type: CardType::T,
                rank: 10,
            }),
            'J' => Ok(Card {
                card_type: CardType::J,
                rank: 11,
            }),
            'Q' => Ok(Card {
                card_type: CardType::Q,
                rank: 12,
            }),
            'K' => Ok(Card {
                card_type: CardType::K,
                rank: 13,
            }),
            'A' => Ok(Card {
                card_type: CardType::A,
                rank: 14,
            }),
            other => {
                let error_message = format!("No card type defined for symbol {}", other);
                Err(error_message)
            }
        }
    }
}

#[derive(Debug)]
enum CombinationType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
pub struct Combination {
    combination_type: CombinationType,
    pub rank: u8,
}

impl Combination {
    pub fn get_combination(cards_string: &str) -> Self {
        let mut card_count: HashMap<char, usize> = HashMap::new();

        for card in cards_string.chars() {
            card_count
                .entry(card)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        let mut counts_vec: Vec<(_, _)> = card_count.into_iter().collect();
        counts_vec.sort_by(|a, b| b.1.cmp(&a.1));

        let (_, first_count) = counts_vec[0];
        let (_, second_count) = counts_vec.get(1).unwrap_or(&('d', 0));

        match (first_count, second_count) {
            (5, 0) => Combination {
                combination_type: CombinationType::FiveOfAKind,
                rank: 7,
            },
            (4, 1) => Combination {
                combination_type: CombinationType::FourOfAKind,
                rank: 6,
            },
            (3, 2) => Combination {
                combination_type: CombinationType::FullHouse,
                rank: 5,
            },
            (3, 1) => Combination {
                combination_type: CombinationType::ThreeOfAKind,
                rank: 4,
            },
            (2, 2) => Combination {
                combination_type: CombinationType::TwoPair,
                rank: 3,
            },
            (2, 1) => Combination {
                combination_type: CombinationType::OnePair,
                rank: 2,
            },
            (1, 1) => Combination {
                combination_type: CombinationType::HighCard,
                rank: 1,
            },
            _ => panic!("Cannot determine combination"),
        }
    }
}
