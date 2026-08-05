//! Export di una lettura come blocco di testo pronto per i social.

use crate::model::{Deck, Reading};

/// Blocco di testo formattato per una caption (carta + significato + hashtag).
pub fn social_text(deck: &Deck, reading: &Reading) -> String {
    let mut out = String::new();
    let date = reading.at.format("%d.%m.%Y");
    out.push_str(&format!("✦ {} — {}\n", reading.deck, date));
    out.push_str(&format!("  {}\n\n", reading.spread.label()));

    let positions = reading.spread.positions();
    for (i, drawn) in reading.cards.iter().enumerate() {
        let pos = positions.get(i).copied().unwrap_or("");
        let card = deck.cards.iter().find(|c| c.name == drawn.card);
        let orient = if drawn.orientation.is_reversed() { " (rovesciata)" } else { "" };

        if !pos.is_empty() {
            out.push_str(&format!("[{pos}]\n"));
        }
        out.push_str(&format!("{}{}\n", drawn.card, orient));
        if let Some(c) = card {
            out.push_str(&format!("{}\n", c.meaning(drawn.orientation)));
        }
        out.push('\n');
    }

    if !reading.note.trim().is_empty() {
        out.push_str(&format!("« {} »\n\n", reading.note.trim()));
    }

    // Hashtag dalle parole chiave delle carte pescate.
    let mut tags = vec![
        "#tarot".to_string(),
        "#oracle".to_string(),
        "#darkart".to_string(),
    ];
    for drawn in &reading.cards {
        if let Some(c) = deck.cards.iter().find(|c| c.name == drawn.card) {
            for kw in &c.keywords {
                let tag = format!("#{}", kw.replace(' ', ""));
                if !tags.contains(&tag) {
                    tags.push(tag);
                }
            }
        }
    }
    out.push_str(&tags.join(" "));
    out.push('\n');
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deck::void_arcana;
    use crate::model::{DrawnCard, Orientation, Spread};
    use chrono::Local;

    fn reading() -> Reading {
        Reading {
            at: Local::now(),
            deck: "Void Arcana".into(),
            spread: Spread::ThreeCard,
            cards: vec![
                DrawnCard { card: "The Null".into(), orientation: Orientation::Upright },
                DrawnCard { card: "The Reboot".into(), orientation: Orientation::Reversed },
                DrawnCard { card: "The Network".into(), orientation: Orientation::Upright },
            ],
            note: "un giorno strano".into(),
            daily: true,
        }
    }

    #[test]
    fn social_text_has_cards_positions_and_hashtags() {
        let deck = void_arcana();
        let text = social_text(&deck, &reading());
        assert!(text.contains("Void Arcana"));
        assert!(text.contains("[Passato]"));
        assert!(text.contains("The Reboot (rovesciata)"));
        assert!(text.contains("un giorno strano"));
        assert!(text.contains("#tarot"));
        // Include un hashtag da una parola chiave delle carte.
        assert!(text.contains("#trasformazione") || text.contains("#connessione"));
    }

    #[test]
    fn reversed_meaning_is_used_in_export() {
        let deck = void_arcana();
        let text = social_text(&deck, &reading());
        let reboot = deck.cards.iter().find(|c| c.name == "The Reboot").unwrap();
        assert!(text.contains(reboot.meaning(Orientation::Reversed)));
    }
}
