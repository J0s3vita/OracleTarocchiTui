//! Modello dati: carte, mazzo, letture, cronologia.

use chrono::{DateTime, Local, NaiveDate};
use serde::{Deserialize, Serialize};

/// Orientamento della carta pescata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Orientation {
    Upright,
    Reversed,
}

impl Orientation {
    pub fn label(self) -> &'static str {
        match self {
            Orientation::Upright => "dritta",
            Orientation::Reversed => "rovesciata",
        }
    }
    pub fn is_reversed(self) -> bool {
        matches!(self, Orientation::Reversed)
    }
}

/// Una carta del mazzo. Il simbolo e' ASCII art multilinea.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub name: String,
    /// Numero/indicatore breve (es. "0", "XIII", "∅"), mostrato in angolo.
    #[serde(default)]
    pub number: String,
    /// Significato da dritta.
    pub upright: String,
    /// Significato da rovesciata.
    pub reversed: String,
    /// Parole chiave, per l'export e i riepiloghi.
    #[serde(default)]
    pub keywords: Vec<String>,
    /// Simbolo ASCII art (righe separate da newline).
    pub symbol: String,
    /// Colore dominante RGB, che tinge i bordi durante la visualizzazione.
    #[serde(default = "default_color")]
    pub color: [u8; 3],
}

fn default_color() -> [u8; 3] {
    [140, 120, 200]
}

impl Card {
    /// Righe del simbolo ASCII.
    pub fn symbol_lines(&self) -> Vec<&str> {
        self.symbol.lines().collect()
    }

    /// Significato per l'orientamento dato.
    pub fn meaning(&self, o: Orientation) -> &str {
        match o {
            Orientation::Upright => &self.upright,
            Orientation::Reversed => &self.reversed,
        }
    }
}

/// Un mazzo: insieme di carte con nome e descrizione.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deck {
    pub name: String,
    #[serde(default)]
    pub description: String,
    /// Probabilita' che una carta esca rovesciata (0..1).
    #[serde(default = "default_reversed_chance")]
    pub reversed_chance: f32,
    pub cards: Vec<Card>,
}

fn default_reversed_chance() -> f32 {
    0.4
}

impl Deck {
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
}

/// Tipo di stesura.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Spread {
    /// Carta singola.
    Single,
    /// Tre carte: passato / presente / futuro.
    ThreeCard,
}

impl Spread {
    pub fn card_count(self) -> usize {
        match self {
            Spread::Single => 1,
            Spread::ThreeCard => 3,
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Spread::Single => "Carta singola",
            Spread::ThreeCard => "Passato · Presente · Futuro",
        }
    }
    /// Etichette di posizione per la stesura.
    pub fn positions(self) -> &'static [&'static str] {
        match self {
            Spread::Single => &["Il segnale"],
            Spread::ThreeCard => &["Passato", "Presente", "Futuro"],
        }
    }
}

/// Una carta pescata dentro una lettura: indice nel mazzo + orientamento.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrawnCard {
    /// Nome della carta (stabile anche se il mazzo viene riordinato).
    pub card: String,
    pub orientation: Orientation,
}

/// Una lettura completa, salvata nella cronologia.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reading {
    pub at: DateTime<Local>,
    pub deck: String,
    pub spread: Spread,
    pub cards: Vec<DrawnCard>,
    #[serde(default)]
    pub note: String,
    /// Vera se e' la pescata "del giorno".
    #[serde(default)]
    pub daily: bool,
}

impl Reading {
    pub fn date(&self) -> NaiveDate {
        self.at.date_naive()
    }

    /// Riga sintetica: "Carta (rovesciata) · Carta · ...".
    pub fn summary(&self) -> String {
        self.cards
            .iter()
            .map(|c| {
                if c.orientation.is_reversed() {
                    format!("{} (rov.)", c.card)
                } else {
                    c.card.clone()
                }
            })
            .collect::<Vec<_>>()
            .join(" · ")
    }
}

/// Contenitore serializzato della cronologia.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct History {
    #[serde(default)]
    pub readings: Vec<Reading>,
}

impl History {
    /// La pescata giornaliera di una certa data, se presente.
    pub fn daily_for(&self, date: NaiveDate) -> Option<&Reading> {
        self.readings
            .iter()
            .find(|r| r.daily && r.date() == date)
    }

    /// Letture dalla piu' recente alla piu' vecchia.
    pub fn recent(&self) -> Vec<&Reading> {
        let mut v: Vec<&Reading> = self.readings.iter().collect();
        v.sort_by_key(|r| std::cmp::Reverse(r.at));
        v
    }

    pub fn push(&mut self, reading: Reading) {
        self.readings.push(reading);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn reading(daily: bool, y: i32, m: u32, d: u32) -> Reading {
        Reading {
            at: Local.with_ymd_and_hms(y, m, d, 12, 0, 0).unwrap(),
            deck: "Void".into(),
            spread: Spread::Single,
            cards: vec![DrawnCard {
                card: "Il Vuoto".into(),
                orientation: Orientation::Reversed,
            }],
            note: String::new(),
            daily,
        }
    }

    #[test]
    fn meaning_switches_with_orientation() {
        let c = Card {
            name: "X".into(),
            number: String::new(),
            upright: "luce".into(),
            reversed: "ombra".into(),
            keywords: vec![],
            symbol: "***".into(),
            color: [1, 2, 3],
        };
        assert_eq!(c.meaning(Orientation::Upright), "luce");
        assert_eq!(c.meaning(Orientation::Reversed), "ombra");
    }

    #[test]
    fn daily_for_finds_the_right_day() {
        let mut h = History::default();
        h.push(reading(true, 2024, 1, 1));
        h.push(reading(false, 2024, 1, 2));
        assert!(h.daily_for(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()).is_some());
        assert!(h.daily_for(NaiveDate::from_ymd_opt(2024, 1, 2).unwrap()).is_none());
    }

    #[test]
    fn summary_marks_reversed() {
        let r = reading(true, 2024, 1, 1);
        assert_eq!(r.summary(), "Il Vuoto (rov.)");
    }

    #[test]
    fn spread_positions_match_count() {
        assert_eq!(Spread::ThreeCard.card_count(), 3);
        assert_eq!(Spread::ThreeCard.positions().len(), 3);
        assert_eq!(Spread::Single.positions().len(), 1);
    }
}
