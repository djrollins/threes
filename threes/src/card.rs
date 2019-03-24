#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Card {
    value: usize,
}

const VALID_CARDS: [usize; 15] = [
    1, 2, 3, 6, 12, 24, 48, 96, 192, 384, 768, 1536, 3072, 6144, 12288,
];

#[derive(Debug, PartialEq)]
pub enum CardError {
    InvalidValue(usize),
}

impl Card {
    pub fn new(value: usize) -> Result<Card, CardError> {
        if VALID_CARDS.contains(&value) {
            Ok(Card { value })
        } else {
            Err(CardError::InvalidValue(value))
        }
    }

    pub fn mergable(&self, other: Card) -> bool {
        match (self.value, other.value) {
            (1, 2) | (2, 1) => true,
            (x, y) => x == y,
        }
    }

    pub fn merge(&self, other: Card) -> Option<Card> {
        if self.mergable(other) {
            let value = self.value + other.value;
            Some(Card { value })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_make_cards_from_valid_values() {
        for value in 1..4 {
            assert_eq!(Card { value }, Card::new(value).unwrap());
        }

        for card_i in 4..13 {
            let value = 3 * (2f64.powi(card_i) as usize);
            assert_eq!(Card { value }, Card::new(value).unwrap());
        }
    }

    #[test]
    fn new_card_with_invalid_value_returns_error() {
        assert_eq!(CardError::InvalidValue(16), Card::new(16).err().unwrap())
    }

    #[test]
    fn can_merge_mergable_cards() {
        for &(r, x, y) in [(3, 1, 2), (3, 2, 1), (6, 3, 3), (48, 24, 24)].iter() {
            assert_eq!(
                Card::new(r).unwrap(),
                Card::new(x).unwrap().merge(Card::new(y).unwrap()).unwrap()
            );
        }
    }

    #[test]
    fn cannot_merge_unmergable_cards() {
        assert_eq!(None, Card::new(6).unwrap().merge(Card::new(3).unwrap()));
    }
}
