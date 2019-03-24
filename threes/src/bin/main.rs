use rand;
use threes::deck::Deck;

fn main() {
    let mut deck = Deck::new(rand::thread_rng());

    for _ in 0..1000 {
        print!("{} ", deck.draw(768));
    }
}
