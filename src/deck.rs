//! Mazzi predefiniti (incorporati) e caricamento di mazzi personalizzati TOML.
//!
//! Il mazzo e' un dato esterno fin dall'inizio: i mazzi built-in sono solo il
//! punto di partenza, l'utente puo' aggiungerne in `~/.oracledecktui/decks/`.

use crate::model::{Card, Deck};

fn card(
    name: &str,
    number: &str,
    upright: &str,
    reversed: &str,
    keywords: &[&str],
    color: [u8; 3],
    symbol: &str,
) -> Card {
    Card {
        name: name.to_string(),
        number: number.to_string(),
        upright: upright.to_string(),
        reversed: reversed.to_string(),
        keywords: keywords.iter().map(|s| s.to_string()).collect(),
        symbol: symbol.trim_matches('\n').to_string(),
        color,
    }
}

/// "Void Arcana" — oracolo originale dark surrealist.
pub fn void_arcana() -> Deck {
    Deck {
        name: "Void Arcana".to_string(),
        description: "Oracolo originale dark surrealist. Nessun arcano è ciò che sembra.".to_string(),
        reversed_chance: 0.4,
        cards: vec![
            card(
                "The Null", "0",
                "Un ciclo si apre nel vuoto. Potenziale puro, nessuna forma ancora scelta.",
                "Paralisi, paura del salto. Il vuoto ti trattiene invece di liberarti.",
                &["inizio", "vuoto", "potenziale"],
                [150, 130, 210],
                r#"
   ╭─────────╮
  ╱ · · · · · ╲
 │ ·   ╲ ╱   · │
 │ ·    ∅    · │
 │ ·   ╱ ╲   · │
  ╲ · · · · · ╱
   ╰─────────╯
"#,
            ),
            card(
                "The Signal", "I",
                "Volontà che si irradia. Ciò che pensi comincia a trasmettersi nel reale.",
                "Rumore, dispersione. Il messaggio si perde prima di arrivare.",
                &["manifestazione", "volontà"],
                [90, 220, 200],
                r#"
   (( ● ))
  ·  ╲│╱  ·
     ╲│╱
     ╱│╲
    ╱ │ ╲
   ═══╪═══
      │
  ▂▂▂▂█▂▂▂▂
"#,
            ),
            card(
                "The Oracle", "II",
                "Sapere che non passa dalle parole. Ascolta il silenzio tra i dati.",
                "Segreti che marciscono. Intuizione soffocata dal dubbio.",
                &["intuizione", "mistero"],
                [120, 120, 230],
                r#"
   ╱▔▔▔▔▔╲
  │ ▄   ▄ │
  │ ◑   ◑ │
  │   ▽   │
   ╲ ▁▁▁ ╱
     ╲ ╱
  ▚▞▚ ▞ ▚▞
"#,
            ),
            card(
                "The Current", "III",
                "Forza che scorre invece di spingere. Potere quieto, ininterrotto.",
                "Cortocircuito. La forza si scarica a vuoto, ti brucia.",
                &["energia", "flusso"],
                [230, 160, 80],
                r#"
      ◈
     ╱⚡╲
    │≈≈≈│
    │≈≈≈│
    │≈≈≈│
     ╲⚡╱
      ◈
"#,
            ),
            card(
                "The Sovereign", "IV",
                "Struttura che regge. Confini scelti, non subiti.",
                "Controllo che diventa gabbia. Rigidità che spezza invece di reggere.",
                &["ordine", "struttura"],
                [200, 90, 90],
                r#"
   ♦ ▟█▙ ♦
    ▟███▙
   ┏━━━━━┓
   ┃ ▐█▌ ┃
   ┃ ▐█▌ ┃
   ┗━━━━━┛
"#,
            ),
            card(
                "The Drift", "V",
                "Movimento senza sforzo verso ciò che ti chiama. Lasciati portare.",
                "Deriva cieca. Ti muovi, ma nessuno tiene il timone.",
                &["momento", "viaggio"],
                [80, 180, 230],
                r#"
     ______
   ╱‾      ‾╲__
  ⟨  ◦    ◦    ⟩
   ╲__      __╱
      ‾‾‾‾‾‾
  ≈ ≈ ≈ ≈ ≈ ≈
"#,
            ),
            card(
                "The Hermit Node", "VI",
                "Ti scolleghi per sentirti. La solitudine come antenna.",
                "Isolamento che diventa muro. Ti perdi nel buio che cercavi.",
                &["solitudine", "ricerca"],
                [130, 140, 160],
                r#"
     ◢██◣
    ╱ ◉◉ ╲
   │      │
    ╲    ╱
     ╲  ╱
      ▼▼
   ·  *  ·
"#,
            ),
            card(
                "The Cycle", "VII",
                "La ruota gira: ciò che è sotto salirà. Nulla resta fermo.",
                "Ti aggrappi al giro sbagliato. Ripeti invece di ruotare.",
                &["destino", "cambiamento"],
                [220, 190, 90],
                r#"
    ╭──◍──╮
   ◍  ╲│╱  ◍
   ├──╳╳╳──┤
   ◍  ╱│╲  ◍
    ╰──◍──╯
"#,
            ),
            card(
                "The Balance", "VIII",
                "Verità fredda, senza sconti. Ciò che pesa, pesa davvero.",
                "Bilancia truccata. Ti menti sul peso delle cose.",
                &["verità", "giustizia"],
                [180, 200, 200],
                r#"
       △
    ───┴───
   ╱   │   ╲
  ▢    │    ▢
   ╲   │   ╱
    ═══╧═══
"#,
            ),
            card(
                "The Suspended", "IX",
                "Ti fermi a testa in giù per vedere il vero. Resa che rivela.",
                "Sacrificio inutile. Stai appeso per niente, ostinato.",
                &["resa", "sospensione"],
                [110, 130, 190],
                r#"
   ════╤════
       │
      ◯◯
     ╱   ╲
     ╲   ╱
      ╲ ╱
       ▽
"#,
            ),
            card(
                "The Reboot", "XIII",
                "Morte come riavvio. Qualcosa deve chiudersi perché il resto giri.",
                "Ti aggrappi al vecchio sistema. Il riavvio rimandato marcisce.",
                &["trasformazione", "fine"],
                [200, 70, 110],
                r#"
     ▟███▙
    ▟ ☠ ▙
   │  ⟳   │
    ▜ ▁ ▛
     ▜███▛
    ▂▂▂▂▂▂▂
"#,
            ),
            card(
                "The Crash", "XVI",
                "La torre cade perché era falsa. Distruzione che libera.",
                "Crollo negato. Punti l'edificio invece di lasciarlo andare.",
                &["collasso", "rivelazione"],
                [240, 100, 60],
                r#"
   *  ▟█▙  *
      █▓█
     ╱█▓█╲
    ╱ █▓█ ╲
  ✦ ▔▔▔▔▔▔▔ ✦
   ░░░░░░░░░
"#,
            ),
            card(
                "The Beacon", "XVII",
                "Un faro nel rumore. Speranza sottile ma sufficiente.",
                "Faro spento. Cerchi luce dove hai smesso di guardare.",
                &["speranza", "guida"],
                [120, 210, 230],
                r#"
       ✦
      ╱♢╲
     ╱ ║ ╲
    ▔▔▔║▔▔▔
       ║
       ║
   ~~~~▚~~~~
"#,
            ),
            card(
                "The Static", "XVIII",
                "La luna trasmette solo statica. Ciò che vedi è per metà tuo.",
                "Inganno che si dissolve. La nebbia si apre, brucia un po'.",
                &["illusione", "inconscio"],
                [90, 100, 150],
                r#"
   ▁▂▃▄▅▄▃▂▁
   ▞▚▞▚▞▚▞▚▞
   ) ▁▂▃▂▁ (
   ▞▚▞▚▞▚▞▚▞
   ▁▂▃▄▅▄▃▂▁
"#,
            ),
            card(
                "The Solar", "XIX",
                "Luce piena, niente ombre a nascondersi. Chiarezza calda.",
                "Sole accecante. Tanta luce che non vedi più nulla.",
                &["chiarezza", "gioia"],
                [240, 200, 80],
                r#"
   ╲   │   ╱
    ╲  │  ╱
   ── (☼) ──
    ╱  │  ╲
   ╱   │   ╲
"#,
            ),
            card(
                "The Network", "XXI",
                "Tutto si connette. Il cerchio si chiude e diventa mondo.",
                "Nodi scollegati. Il tutto si sfilaccia ai bordi.",
                &["completezza", "connessione"],
                [110, 220, 160],
                r#"
   ◍───────◍
   │╲     ╱│
   │ ╲   ╱ │
   │  ╲ ╱  │
   │   ◍   │
   │  ╱ ╲  │
   ◍───────◍
"#,
            ),
        ],
    }
}

/// "Neon Oracle" — mazzo piu' piccolo, per dimostrare la scelta multi-mazzo.
pub fn neon_oracle() -> Deck {
    Deck {
        name: "Neon Oracle".to_string(),
        description: "Oracolo breve al neon: sei frammenti per una risposta rapida.".to_string(),
        reversed_chance: 0.35,
        cards: vec![
            card(
                "Glow", "†",
                "Qualcosa in te si accende. Segui il bagliore.",
                "Luce fredda, imitata. Non tutto ciò che brilla è tuo.",
                &["accensione"],
                [255, 90, 200],
                r#"
   ✦  ✦  ✦
  ✦ ▄▄▄▄▄ ✦
  ✦ █▓▓▓█ ✦
  ✦ ▀▀▀▀▀ ✦
   ✦  ✦  ✦
"#,
            ),
            card(
                "Wire", "‡",
                "Un legame regge. Fidati del filo teso.",
                "Filo scoperto. Il contatto brucia invece di unire.",
                &["connessione"],
                [90, 230, 255],
                r#"
  ●╌╌╌╌╌╌╌●
   ╲     ╱
    ╲   ╱
     ╲ ╱
      ╳
     ╱ ╲
  ●╌╌╌╌╌╌╌●
"#,
            ),
            card(
                "Hex", "◇",
                "Uno schema nascosto ti protegge. Non tutto è caos.",
                "Schema rotto. La protezione ha una crepa.",
                &["pattern"],
                [180, 120, 255],
                r#"
   ⬡ ⬡ ⬡ ⬡
  ⬡ ⬡ ⬡ ⬡ ⬡
   ⬡ ⬢ ⬢ ⬡
  ⬡ ⬡ ⬡ ⬡ ⬡
   ⬡ ⬡ ⬡ ⬡
"#,
            ),
            card(
                "Fade", "◈",
                "Lascia sfumare ciò che ha fatto il suo tempo.",
                "Ti dissolvi troppo presto. Resta ancora un momento.",
                &["dissolvenza"],
                [130, 150, 190],
                r#"
  ███████████
  ▓▓▓▓▓▓▓▓▓▓▓
  ▒▒▒▒▒▒▒▒▒▒▒
  ░░░░░░░░░░░
  · · · · · ·
"#,
            ),
            card(
                "Spark", "✧",
                "Un'idea improvvisa. Coglila prima che svanisca.",
                "Scintilla a vuoto. Tanta energia, nessuna direzione.",
                &["intuizione"],
                [255, 210, 90],
                r#"
      ╱✧╲
     ╱ │ ╲
    ╱  │  ╲
   ✧───┼───✧
    ╲  │  ╱
     ╲ │ ╱
      ╲✧╱
"#,
            ),
            card(
                "Echo", "◐",
                "Qualcosa che hai detto torna a te. Ascoltalo.",
                "Eco che non finisce. Ripeti un passato che è passato.",
                &["ritorno"],
                [200, 120, 160],
                r#"
  ◉ ⟩ ⟩ ⟩ ⟩
  ◉ ⟩ ⟩ ⟩
  ◉ ⟩ ⟩
  ◉ ⟩ ⟩ ⟩
  ◉ ⟩ ⟩ ⟩ ⟩
"#,
            ),
        ],
    }
}

/// Tutti i mazzi incorporati.
pub fn builtin_decks() -> Vec<Deck> {
    vec![void_arcana(), neon_oracle()]
}

/// Serializza un mazzo in TOML (per l'export/salvataggio dei mazzi editati).
pub fn to_toml(deck: &Deck) -> Result<String, String> {
    toml::to_string_pretty(deck).map_err(|e| e.to_string())
}

/// Deserializza un mazzo da TOML, validando che non sia vuoto.
pub fn from_toml(text: &str) -> Result<Deck, String> {
    let deck: Deck = toml::from_str(text).map_err(|e| e.message().to_string())?;
    if deck.cards.is_empty() {
        return Err("il mazzo non contiene carte".into());
    }
    Ok(deck)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_decks_are_non_empty_and_well_formed() {
        for deck in builtin_decks() {
            assert!(!deck.cards.is_empty(), "{} è vuoto", deck.name);
            for c in &deck.cards {
                assert!(!c.name.is_empty());
                assert!(!c.upright.is_empty(), "{} senza dritto", c.name);
                assert!(!c.reversed.is_empty(), "{} senza rovescio", c.name);
                assert!(!c.symbol_lines().is_empty(), "{} senza simbolo", c.name);
            }
        }
    }

    #[test]
    fn deck_survives_a_toml_roundtrip() {
        let deck = void_arcana();
        let text = to_toml(&deck).unwrap();
        let back = from_toml(&text).unwrap();
        assert_eq!(back.name, deck.name);
        assert_eq!(back.cards.len(), deck.cards.len());
        assert_eq!(back.cards[0].name, deck.cards[0].name);
    }

    #[test]
    fn empty_deck_is_rejected() {
        let toml = "name = \"Vuoto\"\nreversed_chance = 0.4\ncards = []\n";
        assert!(from_toml(toml).is_err());
    }

    #[test]
    fn symbols_are_reasonably_sized() {
        for deck in builtin_decks() {
            for c in &deck.cards {
                let lines = c.symbol_lines();
                assert!(lines.len() <= 9, "{} troppo alto", c.name);
                assert!(lines.iter().all(|l| l.chars().count() <= 20), "{} troppo largo", c.name);
            }
        }
    }
}
