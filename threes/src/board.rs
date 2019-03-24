use crate::card::Card;
use rand::Rng;

pub struct Board {
    grid: [Option<Card>; 16],
}

impl Board {
    fn new<R: Rng>(cards: &[Card; 9], rng: R) -> Board {
        let grid: [None; 16];

        Board { grid }
    }
}
