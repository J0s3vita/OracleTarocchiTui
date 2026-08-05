//! Logica di pescaggio: estrazione carte, orientamento, stesure.

use crate::model::{Deck, DrawnCard, Orientation};
use crate::rng::Rng;

/// Pesca `n` carte distinte dal mazzo, ciascuna con un orientamento.
///
/// Se il mazzo ha meno di `n` carte, ne restituisce quante può (senza ripetere).
pub fn draw(deck: &Deck, n: usize, rng: &mut Rng) -> Vec<DrawnCard> {
    let mut indices: Vec<usize> = (0..deck.cards.len()).collect();
    rng.shuffle(&mut indices);
    indices
        .into_iter()
        .take(n)
        .map(|i| DrawnCard {
            card: deck.cards[i].name.clone(),
            orientation: if rng.bool_with(deck.reversed_chance) {
                Orientation::Reversed
            } else {
                Orientation::Upright
            },
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deck::void_arcana;

    #[test]
    fn draws_requested_number_without_repeats() {
        let deck = void_arcana();
        let mut rng = Rng::with_seed(1);
        let drawn = draw(&deck, 3, &mut rng);
        assert_eq!(drawn.len(), 3);
        let names: std::collections::HashSet<_> = drawn.iter().map(|d| &d.card).collect();
        assert_eq!(names.len(), 3, "nessuna carta ripetuta nella stesura");
    }

    #[test]
    fn same_seed_same_draw() {
        let deck = void_arcana();
        let a = draw(&deck, 3, &mut Rng::with_seed(99));
        let b = draw(&deck, 3, &mut Rng::with_seed(99));
        let names_a: Vec<_> = a.iter().map(|d| (&d.card, d.orientation)).collect();
        let names_b: Vec<_> = b.iter().map(|d| (&d.card, d.orientation)).collect();
        assert_eq!(names_a, names_b);
    }

    #[test]
    fn never_reversed_when_chance_zero() {
        let mut deck = void_arcana();
        deck.reversed_chance = 0.0;
        let drawn = draw(&deck, deck.cards.len(), &mut Rng::with_seed(5));
        assert!(drawn.iter().all(|d| d.orientation == Orientation::Upright));
    }

    #[test]
    fn always_reversed_when_chance_one() {
        let mut deck = void_arcana();
        deck.reversed_chance = 1.0;
        let drawn = draw(&deck, 4, &mut Rng::with_seed(5));
        assert!(drawn.iter().all(|d| d.orientation == Orientation::Reversed));
    }

    #[test]
    fn drawing_more_than_deck_size_caps() {
        let deck = void_arcana();
        let drawn = draw(&deck, deck.cards.len() + 10, &mut Rng::with_seed(3));
        assert_eq!(drawn.len(), deck.cards.len());
    }
}
