use rand::seq::SliceRandom;
use std::cell::RefCell;
use std::ops::Deref;

pub type Card = usize;

pub struct Deck<R> {
    basic_deck: BasicDeck,
    rng: R,
}

impl<R: rand::Rng> Deck<R> {
    pub fn new(mut rng: R) -> Deck<R> {
        let basic_deck = BasicDeck::new(&mut rng);

        Deck { basic_deck, rng }
    }

    pub fn draw(&mut self, high_card: Card) -> Card {
        if let Some(bonus) = self.draw_bonus(high_card) {
            bonus
        } else {
            self.draw_basic()
        }
    }

    fn draw_basic(&mut self) -> Card {
        if let Some(card) = self.basic_deck.next() {
            return card;
        }

        self.basic_deck = BasicDeck::new(&mut self.rng);
        self.basic_deck.next().unwrap()
    }

    fn draw_bonus(&mut self, high_card: Card) -> Option<Card> {
        if !self.rng.gen_bool(1.0 / 21.0) {
            return None;
        }

        bonus_cards(high_card).map(|pool| {
            // Yuck!
            *(&pool[..]).choose(&mut self.rng).unwrap()
        })
    }
}

struct BasicDeck {
    cards: [Card; 12],
    next: usize,
}

impl BasicDeck {
    fn new<R: rand::Rng>(rng: &mut R) -> BasicDeck {
        let mut cards = [1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3];
        cards.shuffle(rng);

        BasicDeck { cards, next: 0 }
    }
}

impl Iterator for BasicDeck {
    type Item = Card;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next == self.cards.len() {
            None
        } else {
            let card = self.cards[self.next];
            self.next += 1;
            Some(card)
        }
    }
}

#[derive(Debug, PartialEq)]
struct BonusPool {
    count: usize,
    cards: [Card; 3],
}

impl Deref for BonusPool {
    type Target = [Card];

    fn deref(&self) -> &Self::Target {
        &self.cards[0..self.count]
    }
}

fn bonus_cards(high_card: Card) -> Option<BonusPool> {
    if high_card < 48 {
        return None;
    }

    let highest = high_card / 8;
    let count = std::cmp::min(high_card / 48, 3);
    let cards = [highest, highest / 2, highest / 4];

    Some(BonusPool { count, cards })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bonus_cards_returns_none_if_high_card_less_than_48() {
        assert_eq!(None, bonus_cards(24));
    }

    #[test]
    fn bonus_cards_returns_6_when_highcard_is_48() {
        assert_eq!(&[6], &(bonus_cards(48).unwrap())[..]);
    }

    #[test]
    fn bonus_cards_returns_6_and_12_when_highcard_is_96() {
        assert_eq!(&[12, 6], &(bonus_cards(96).unwrap())[..]);
    }

    #[test]
    fn bonus_cards_returns_three_cards_when_high_card_is_high_enough() {
        assert_eq!(&[48, 24, 12], &(bonus_cards(384).unwrap())[..]);
        assert_eq!(&[96, 48, 24], &(bonus_cards(768).unwrap())[..]);
        assert_eq!(&[192, 96, 48], &(bonus_cards(1536).unwrap())[..]);
    }
}
