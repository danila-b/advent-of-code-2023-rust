use crate::card::{Card, Combination};
use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Debug)]
pub struct Hand {
    cards: Vec<Card>,
    combination: Combination,
    pub bid: usize,
}

impl Hand {
    fn parse_cards(cards: &str) -> Vec<Card> {
        let mut parsed_cards: Vec<Card> = vec![];

        for card in cards.chars() {
            let card_var = Card::from_char(&card).unwrap();
            parsed_cards.push(card_var);
        }

        parsed_cards
    }
}

#[derive(Debug)]
pub struct ParseHandFromStringError;

impl FromStr for Hand {
    type Err = ParseHandFromStringError;

    fn from_str(s: &str) -> Result<Self, ParseHandFromStringError> {
        let (cards, bid) = s.split_once(' ').unwrap();
        let bid: usize = bid.parse::<usize>().unwrap();
        let combination: Combination = Combination::get_combination(cards);
        let cards = Hand::parse_cards(cards);

        Ok(Hand {
            cards: cards,
            combination: combination,
            bid: bid,
        })
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.combination.rank != other.combination.rank
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.combination.rank < other.combination.rank {
            Some(Ordering::Less)
        } else if self.combination.rank > other.combination.rank {
            Some(Ordering::Greater)
        } else {
            for card_index in 0..self.cards.len() {
                if self.cards[card_index].rank < other.cards[card_index].rank {
                    return Some(Ordering::Less);
                } else if self.cards[card_index].rank > other.cards[card_index].rank {
                    return Some(Ordering::Greater);
                } else {
                    continue;
                }
            }

            Some(Ordering::Equal)
        }
    }
}
