//! Persistenza locale: mazzi personalizzati, cronologia letture, pescata del giorno.
//!
//! Layout:
//! ```text
//! ~/.oracledecktui/
//!   decks/<slug>.toml     mazzi personalizzati (oltre a quelli incorporati)
//!   history.json          cronologia di tutte le letture
//! ```
//! `ORACLE_DIR` sposta la cartella dati.

use std::fs;
use std::io;
use std::path::PathBuf;

use crate::deck;
use crate::model::{Deck, History};

#[cfg(test)]
thread_local! {
    /// Cartella dati isolata per il test corrente (ogni test = un thread).
    static TEST_DIR: std::cell::RefCell<Option<PathBuf>> = const { std::cell::RefCell::new(None) };
}

#[cfg(test)]
pub fn set_test_dir(path: PathBuf) {
    let _ = fs::remove_dir_all(&path);
    TEST_DIR.with(|d| *d.borrow_mut() = Some(path));
}

pub fn data_dir() -> PathBuf {
    #[cfg(test)]
    if let Some(dir) = TEST_DIR.with(|d| d.borrow().clone()) {
        return dir;
    }
    if let Some(dir) = std::env::var_os("ORACLE_DIR") {
        return PathBuf::from(dir);
    }
    let base = dirs_home();
    base.join(".oracledecktui")
}

fn dirs_home() -> PathBuf {
    // Evitiamo una dipendenza extra: HOME e' sufficiente su Unix/macOS.
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
}

pub fn decks_dir() -> PathBuf {
    data_dir().join("decks")
}

pub fn history_path() -> PathBuf {
    data_dir().join("history.json")
}

pub fn ensure_dirs() -> io::Result<()> {
    fs::create_dir_all(decks_dir())
}

fn slug(name: &str) -> String {
    let mut out = String::new();
    let mut dash = false;
    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            dash = false;
        } else if !dash && !out.is_empty() {
            out.push('-');
            dash = true;
        }
    }
    let t = out.trim_end_matches('-').to_string();
    if t.is_empty() { "deck".into() } else { t }
}

/// Carica tutti i mazzi: prima gli incorporati, poi quelli personalizzati da
/// disco (che possono sovrascrivere per nome). Restituisce anche eventuali
/// warning sui file illeggibili.
pub fn load_decks() -> (Vec<Deck>, Vec<String>) {
    let mut decks = deck::builtin_decks();
    let mut warnings = Vec::new();

    if let Ok(entries) = fs::read_dir(decks_dir()) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("toml") {
                continue;
            }
            match fs::read_to_string(&path).map_err(|e| e.to_string()).and_then(|s| deck::from_toml(&s)) {
                Ok(custom) => {
                    // Un mazzo personalizzato con lo stesso nome sostituisce l'incorporato.
                    if let Some(slot) = decks.iter_mut().find(|d| d.name == custom.name) {
                        *slot = custom;
                    } else {
                        decks.push(custom);
                    }
                }
                Err(e) => warnings.push(format!(
                    "{}: {e}",
                    path.file_name().unwrap_or_default().to_string_lossy()
                )),
            }
        }
    }
    (decks, warnings)
}

/// Percorso del file TOML che definisce il mazzo dato (se salvato a mano).
pub fn deck_file(name: &str) -> PathBuf {
    decks_dir().join(format!("{}.toml", slug(name)))
}

/// Elimina il file TOML di un mazzo personalizzato, se esiste.
///
/// Restituisce `true` se un file è stato rimosso, `false` se il mazzo è solo
/// incorporato (non ha file su disco) e quindi non eliminabile.
pub fn delete_deck(name: &str) -> Result<bool, String> {
    let path = deck_file(name);
    if path.exists() {
        fs::remove_file(&path).map_err(|e| e.to_string())?;
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Salva un mazzo personalizzato su disco in TOML.
pub fn save_deck(deck: &Deck) -> Result<PathBuf, String> {
    ensure_dirs().map_err(|e| e.to_string())?;
    let path = decks_dir().join(format!("{}.toml", slug(&deck.name)));
    let text = deck::to_toml(deck)?;
    fs::write(&path, text).map_err(|e| e.to_string())?;
    Ok(path)
}

pub fn load_history() -> (History, Vec<String>) {
    let path = history_path();
    match fs::read_to_string(&path) {
        Ok(text) => match serde_json::from_str::<History>(&text) {
            Ok(h) => (h, Vec::new()),
            Err(e) => (History::default(), vec![format!("history.json: {e}")]),
        },
        Err(e) if e.kind() == io::ErrorKind::NotFound => (History::default(), Vec::new()),
        Err(e) => (History::default(), vec![format!("history.json: {e}")]),
    }
}

pub fn save_history(history: &History) -> Result<(), String> {
    ensure_dirs().map_err(|e| e.to_string())?;
    let text = serde_json::to_string_pretty(history).map_err(|e| e.to_string())?;
    let path = history_path();
    let tmp = path.with_extension("json.tmp");
    fs::write(&tmp, text).map_err(|e| e.to_string())?;
    fs::rename(&tmp, &path).map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{DrawnCard, Orientation, Reading, Spread};
    use chrono::Local;

    fn temp(tag: &str) {
        set_test_dir(std::env::temp_dir().join(format!("oracle-test-{tag}")));
    }

    #[test]
    fn slug_is_safe() {
        assert_eq!(slug("Void Arcana"), "void-arcana");
        assert_eq!(slug("***"), "deck");
    }

    #[test]
    fn load_decks_includes_builtins() {
        temp("builtins");
        let (decks, warnings) = load_decks();
        assert!(warnings.is_empty());
        assert!(decks.iter().any(|d| d.name == "Void Arcana"));
        assert!(decks.iter().any(|d| d.name == "Neon Oracle"));
    }

    #[test]
    fn custom_deck_roundtrips_and_overrides_builtin() {
        temp("custom");
        let mut deck = crate::deck::neon_oracle();
        deck.description = "modificato".into();
        save_deck(&deck).unwrap();

        let (decks, warnings) = load_decks();
        assert!(warnings.is_empty());
        let neon = decks.iter().find(|d| d.name == "Neon Oracle").unwrap();
        assert_eq!(neon.description, "modificato", "il custom sovrascrive l'incorporato");
    }

    #[test]
    fn history_save_then_load() {
        temp("history");
        let mut h = History::default();
        h.push(Reading {
            at: Local::now(),
            deck: "Void Arcana".into(),
            spread: Spread::Single,
            cards: vec![DrawnCard { card: "The Null".into(), orientation: Orientation::Upright }],
            note: "prima lettura".into(),
            daily: true,
        });
        save_history(&h).unwrap();

        let (back, warnings) = load_history();
        assert!(warnings.is_empty());
        assert_eq!(back.readings.len(), 1);
        assert_eq!(back.readings[0].note, "prima lettura");
    }
}
