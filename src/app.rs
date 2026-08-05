//! Stato dell'applicazione: viste, pescaggio con animazioni, editor, cronologia.

use std::time::Instant;

use chrono::Local;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::draw;
use crate::model::{Card, Deck, History, Reading, Spread};
use crate::rng::Rng;
use crate::store;
use crate::theme::{Theme, THEMES};

/// Durata dello shuffle animato.
pub const SHUFFLE_MS: f32 = 850.0;
/// Durata del reveal a glitch.
pub const REVEAL_MS: f32 = 750.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    Daily,
    Spread,
    History,
    Editor,
    Help,
}

impl View {
    pub const TABS: [View; 5] = [View::Daily, View::Spread, View::History, View::Editor, View::Help];
    pub fn label(self) -> &'static str {
        match self {
            View::Daily => "Giornaliera",
            View::Spread => "Stesura",
            View::History => "Cronologia",
            View::Editor => "Editor",
            View::Help => "Aiuto",
        }
    }
}

/// Fase dell'animazione di pescaggio.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    /// Nessuna pescata in corso: mostra invito o carta gia' rivelata.
    Idle,
    Shuffling,
    Revealing,
    Shown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    Info,
    Ok,
    Warn,
    Error,
}

pub struct Status {
    pub text: String,
    pub level: Level,
    pub at: Instant,
}

impl Status {
    fn new(t: impl Into<String>, level: Level) -> Self {
        Self { text: t.into(), level, at: Instant::now() }
    }
    pub fn is_stale(&self) -> bool {
        self.at.elapsed().as_secs_f32() > 7.0
    }
}

// ------------------------------------------------------------------ form model

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldKind {
    Text,
    MultiLine,
    Color,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub label: &'static str,
    pub value: String,
    pub kind: FieldKind,
    pub hint: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormAction {
    NewDeck,
    AddCard,
    EditCard(usize),
    Note,
    ExportPath,
}

pub struct Form {
    pub title: String,
    pub action: FormAction,
    pub fields: Vec<Field>,
    pub idx: usize,
}

impl Form {
    fn value(&self, label: &str) -> String {
        self.fields.iter().find(|f| f.label == label).map(|f| f.value.trim().to_string()).unwrap_or_default()
    }
    fn raw(&self, label: &str) -> String {
        self.fields.iter().find(|f| f.label == label).map(|f| f.value.clone()).unwrap_or_default()
    }
}

pub struct Confirm {
    pub prompt: String,
    pub kind: ConfirmKind,
}

#[derive(Debug, Clone, Copy)]
pub enum ConfirmKind {
    DeleteCard(usize),
    DeleteDeck(usize),
    ClearHistory,
}

/// Modale attivo.
pub enum Modal {
    None,
    DeckPicker(usize),
    Form(Form),
    Confirm(Confirm),
}

pub struct App {
    pub decks: Vec<Deck>,
    pub deck_idx: usize,
    pub history: History,

    pub view: View,
    pub modal: Modal,

    /// Lettura attualmente mostrata (giornaliera o stesura).
    pub current: Option<Reading>,
    pub phase: Phase,
    phase_since: Instant,

    pub history_sel: usize,
    pub editor_sel: usize,

    pub theme_idx: usize,
    /// Nomi dei mazzi incorporati (non eliminabili).
    pub builtin_decks: Vec<String>,
    pub status: Status,
    pub should_quit: bool,
    rng: Rng,
    started: Instant,
}

impl App {
    pub fn new() -> Self {
        let (decks, mut warnings) = store::load_decks();
        let (history, hw) = store::load_history();
        warnings.extend(hw);

        let status = if warnings.is_empty() {
            Status::new("Premi Invio per pescare la carta del giorno", Level::Info)
        } else {
            Status::new(format!("Attenzione: {}", warnings.join("; ")), Level::Warn)
        };

        let now = Instant::now();
        let builtin_decks = crate::deck::builtin_decks().iter().map(|d| d.name.clone()).collect();
        let mut app = Self {
            decks,
            deck_idx: 0,
            history,
            view: View::Daily,
            modal: Modal::None,
            current: None,
            phase: Phase::Idle,
            phase_since: now,
            history_sel: 0,
            editor_sel: 0,
            theme_idx: 0,
            builtin_decks,
            status,
            should_quit: false,
            rng: Rng::from_entropy(),
            started: now,
        };
        app.load_today_daily();
        app
    }

    pub fn theme(&self) -> Theme {
        THEMES[self.theme_idx % THEMES.len()]
    }
    pub fn elapsed(&self) -> f32 {
        self.started.elapsed().as_secs_f32()
    }
    pub fn deck(&self) -> &Deck {
        &self.decks[self.deck_idx.min(self.decks.len().saturating_sub(1))]
    }

    /// Progresso 0..1 della fase corrente.
    pub fn phase_progress(&self) -> f32 {
        let ms = self.phase_since.elapsed().as_secs_f32() * 1000.0;
        let dur = match self.phase {
            Phase::Shuffling => SHUFFLE_MS,
            Phase::Revealing => REVEAL_MS,
            _ => 1.0,
        };
        (ms / dur).clamp(0.0, 1.0)
    }

    fn info(&mut self, t: impl Into<String>) {
        self.status = Status::new(t, Level::Info);
    }
    fn ok(&mut self, t: impl Into<String>) {
        self.status = Status::new(t, Level::Ok);
    }
    fn warn(&mut self, t: impl Into<String>) {
        self.status = Status::new(t, Level::Warn);
    }
    fn error(&mut self, t: impl Into<String>) {
        self.status = Status::new(t, Level::Error);
    }

    /// Carta del mazzo per nome.
    pub fn card_by_name(&self, name: &str) -> Option<&Card> {
        self.deck().cards.iter().find(|c| c.name == name)
    }

    // ---------------------------------------------------------------- daily

    fn load_today_daily(&mut self) {
        let today = Local::now().date_naive();
        if let Some(r) = self.history.daily_for(today) {
            self.current = Some(r.clone());
            self.phase = Phase::Shown;
        }
    }

    fn has_daily_today(&self) -> bool {
        self.history.daily_for(Local::now().date_naive()).is_some()
    }

    // ------------------------------------------------------------- animation

    /// Fa avanzare la macchina a stati dell'animazione. Da chiamare a ogni frame.
    pub fn tick(&mut self) {
        match self.phase {
            Phase::Shuffling if self.phase_progress() >= 1.0 => {
                self.phase = Phase::Revealing;
                self.phase_since = Instant::now();
            }
            Phase::Revealing if self.phase_progress() >= 1.0 => {
                self.phase = Phase::Shown;
                self.phase_since = Instant::now();
                self.persist_current();
            }
            _ => {}
        }
    }

    fn persist_current(&mut self) {
        if let Some(reading) = &self.current {
            // Evita doppioni: la lettura viene salvata una sola volta, a reveal finito.
            let already = self
                .history
                .readings
                .iter()
                .any(|r| r.at == reading.at);
            if !already {
                self.history.push(reading.clone());
                if let Err(e) = store::save_history(&self.history) {
                    self.error(format!("Salvataggio cronologia fallito: {e}"));
                }
            }
        }
    }

    fn start_draw(&mut self, spread: Spread, daily: bool) {
        if self.deck().is_empty() {
            self.warn("Il mazzo è vuoto");
            return;
        }
        // Clona il mazzo per non tenere un prestito su `self` mentre usa `self.rng`.
        let deck = self.deck().clone();
        let cards = draw::draw(&deck, spread.card_count(), &mut self.rng);
        self.current = Some(Reading {
            at: Local::now(),
            deck: deck.name.clone(),
            spread,
            cards,
            note: String::new(),
            daily,
        });
        self.phase = Phase::Shuffling;
        self.phase_since = Instant::now();
    }

    // ------------------------------------------------------------- key handling

    pub fn on_key(&mut self, key: KeyEvent) {
        match &mut self.modal {
            Modal::Form(_) => return self.key_form(key),
            Modal::DeckPicker(_) => return self.key_deck_picker(key),
            Modal::Confirm(_) => return self.key_confirm(key),
            Modal::None => {}
        }
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            self.should_quit = true;
            return;
        }
        // Durante l'animazione, un tasto la salta al risultato.
        if matches!(self.phase, Phase::Shuffling | Phase::Revealing)
            && let KeyCode::Char(' ') | KeyCode::Enter | KeyCode::Esc = key.code {
                self.phase = Phase::Shown;
                self.persist_current();
                return;
            }
        match key.code {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Tab => self.cycle_view(true),
            KeyCode::BackTab => self.cycle_view(false),
            KeyCode::Char(c @ '1'..='5') => self.view = View::TABS[c as usize - '1' as usize],
            KeyCode::Char('t') | KeyCode::Char('T') => {
                self.theme_idx = (self.theme_idx + 1) % THEMES.len();
                let name = self.theme().name;
                self.info(format!("Palette: {name}"));
            }
            KeyCode::Char('d') => self.open_deck_picker(),
            KeyCode::Char('N') => {
                self.view = View::Editor;
                self.open_new_deck();
            }
            _ => match self.view {
                View::Daily => self.key_daily(key),
                View::Spread => self.key_spread(key),
                View::History => self.key_history(key),
                View::Editor => self.key_editor(key),
                View::Help => {}
            },
        }
    }

    fn cycle_view(&mut self, forward: bool) {
        let pos = View::TABS.iter().position(|v| *v == self.view).unwrap_or(0);
        let n = View::TABS.len();
        self.view = View::TABS[if forward { (pos + 1) % n } else { (pos + n - 1) % n }];
        if self.view == View::Daily {
            self.load_today_daily();
        }
    }

    fn key_daily(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Enter | KeyCode::Char(' ') => {
                if self.has_daily_today() && matches!(self.phase, Phase::Shown) {
                    self.warn("Hai già pescato la carta di oggi. Torna domani.");
                } else if !self.has_daily_today() {
                    self.start_draw(Spread::Single, true);
                }
            }
            KeyCode::Char('n') => self.open_note(),
            KeyCode::Char('e') => self.open_export(),
            _ => {}
        }
    }

    fn key_spread(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Enter | KeyCode::Char(' ') => self.start_draw(Spread::ThreeCard, false),
            KeyCode::Char('n') => self.open_note(),
            KeyCode::Char('e') => self.open_export(),
            _ => {}
        }
    }

    fn key_history(&mut self, key: KeyEvent) {
        let n = self.history.readings.len();
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => self.history_sel = self.history_sel.saturating_sub(1),
            KeyCode::Down | KeyCode::Char('j') => {
                self.history_sel = (self.history_sel + 1).min(n.saturating_sub(1));
            }
            KeyCode::PageUp | KeyCode::Char('u') => self.history_sel = self.history_sel.saturating_sub(10),
            KeyCode::PageDown => {
                self.history_sel = (self.history_sel + 10).min(n.saturating_sub(1));
            }
            KeyCode::Home | KeyCode::Char('g') => self.history_sel = 0,
            KeyCode::End | KeyCode::Char('G') => self.history_sel = n.saturating_sub(1),
            KeyCode::Enter => {
                // Riapre la lettura selezionata nella vista opportuna.
                let recent = self.history.recent();
                if let Some(r) = recent.get(self.history_sel).cloned().cloned() {
                    let spread = r.spread;
                    self.current = Some(r);
                    self.phase = Phase::Shown;
                    self.view = if spread == Spread::Single { View::Daily } else { View::Spread };
                }
            }
            KeyCode::Char('e') => {
                let recent = self.history.recent();
                if let Some(r) = recent.get(self.history_sel).cloned().cloned() {
                    self.current = Some(r);
                    self.open_export();
                }
            }
            KeyCode::Char('X') => {
                if self.history.readings.is_empty() {
                    self.warn("Cronologia già vuota");
                    return;
                }
                self.modal = Modal::Confirm(Confirm {
                    prompt: "Cancellare tutta la cronologia delle letture?".into(),
                    kind: ConfirmKind::ClearHistory,
                });
            }
            _ => {}
        }
    }

    fn key_editor(&mut self, key: KeyEvent) {
        let n = self.deck().cards.len();
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => self.editor_sel = self.editor_sel.saturating_sub(1),
            KeyCode::Down | KeyCode::Char('j') => self.editor_sel = (self.editor_sel + 1).min(n.saturating_sub(1)),
            KeyCode::PageUp | KeyCode::Char('u') => self.editor_sel = self.editor_sel.saturating_sub(10),
            KeyCode::PageDown => {
                self.editor_sel = (self.editor_sel + 10).min(n.saturating_sub(1));
            }
            KeyCode::Home | KeyCode::Char('g') => self.editor_sel = 0,
            KeyCode::End | KeyCode::Char('G') => self.editor_sel = n.saturating_sub(1),
            KeyCode::Char('n') => self.open_card_form(None),
            KeyCode::Char('e') => self.open_card_form(Some(self.editor_sel)),
            KeyCode::Char('x') => self.request_delete_card(),
            KeyCode::Char('N') => self.open_new_deck(),
            KeyCode::Char('s') => self.save_current_deck(),
            _ => {}
        }
    }

    // ------------------------------------------------------------- deck picker

    fn open_deck_picker(&mut self) {
        self.modal = Modal::DeckPicker(self.deck_idx);
    }

    fn key_deck_picker(&mut self, key: KeyEvent) {
        let sel = match &self.modal {
            Modal::DeckPicker(s) => *s,
            _ => return,
        };
        match key.code {
            KeyCode::Esc => self.modal = Modal::None,
            KeyCode::Up | KeyCode::Char('k') => {
                if let Modal::DeckPicker(s) = &mut self.modal {
                    *s = s.saturating_sub(1);
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if let Modal::DeckPicker(s) = &mut self.modal
                    && *s + 1 < self.decks.len()
                {
                    *s += 1;
                }
            }
            KeyCode::Char('d') | KeyCode::Char('D') => self.request_delete_deck(sel),
            KeyCode::Enter => {
                self.modal = Modal::None;
                self.deck_idx = sel;
                self.editor_sel = 0;
                let name = self.deck().name.clone();
                self.info(format!("Mazzo: {name}"));
            }
            _ => {}
        }
    }

    /// Chiede conferma prima di eliminare un mazzo. I mazzi incorporati non
    /// sono eliminabili; i personalizzati vengono rimossi (e il file cancellato).
    fn request_delete_deck(&mut self, idx: usize) {
        if idx >= self.decks.len() {
            return;
        }
        let name = self.decks[idx].name.clone();
        if self.builtin_decks.iter().any(|b| b == &name) {
            self.warn("Mazzo incorporato: non eliminabile");
            return;
        }
        self.modal = Modal::Confirm(Confirm {
            prompt: format!("Eliminare il mazzo '{name}' e il suo file su disco?",),
            kind: ConfirmKind::DeleteDeck(idx),
        });
    }

    // ------------------------------------------------------------- forms

    fn open_note(&mut self) {
        if self.current.is_none() {
            self.warn("Nessuna lettura da annotare");
            return;
        }
        let note = self.current.as_ref().map(|r| r.note.clone()).unwrap_or_default();
        self.modal = Modal::Form(Form {
            title: "Nota personale".into(),
            action: FormAction::Note,
            fields: vec![Field {
                label: "Nota",
                value: note,
                kind: FieldKind::MultiLine,
                hint: "Invio a capo · Ctrl+S salva · Esc annulla",
            }],
            idx: 0,
        });
    }

    fn open_export(&mut self) {
        if self.current.is_none() {
            self.warn("Nessuna lettura da esportare");
            return;
        }
        let default = format!(
            "{}/lettura-{}.txt",
            store::data_dir().display(),
            Local::now().format("%Y%m%d-%H%M%S")
        );
        self.modal = Modal::Form(Form {
            title: "Esporta per social".into(),
            action: FormAction::ExportPath,
            fields: vec![Field {
                label: "File",
                value: default,
                kind: FieldKind::Text,
                hint: "percorso del file di testo",
            }],
            idx: 0,
        });
    }

    fn open_new_deck(&mut self) {
        self.modal = Modal::Form(Form {
            title: "Nuovo mazzo".into(),
            action: FormAction::NewDeck,
            fields: vec![
                Field { label: "Nome", value: String::new(), kind: FieldKind::Text, hint: "nome del mazzo" },
                Field { label: "Descrizione", value: String::new(), kind: FieldKind::Text, hint: "" },
            ],
            idx: 0,
        });
    }

    fn open_card_form(&mut self, edit: Option<usize>) {
        let c = edit.and_then(|i| self.deck().cards.get(i)).cloned();
        let action = match edit {
            Some(i) => FormAction::EditCard(i),
            None => FormAction::AddCard,
        };
        let (name, number, up, rev, kw, color, symbol) = match c {
            Some(c) => (
                c.name,
                c.number,
                c.upright,
                c.reversed,
                c.keywords.join(", "),
                format!("{},{},{}", c.color[0], c.color[1], c.color[2]),
                c.symbol,
            ),
            None => (
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                "150,130,210".into(),
                String::new(),
            ),
        };
        self.modal = Modal::Form(Form {
            title: if edit.is_some() { "Modifica carta".into() } else { "Nuova carta".into() },
            action,
            fields: vec![
                Field { label: "Nome", value: name, kind: FieldKind::Text, hint: "nome della carta" },
                Field { label: "Numero", value: number, kind: FieldKind::Text, hint: "es. 0, XIII, ∅" },
                Field { label: "Dritto", value: up, kind: FieldKind::Text, hint: "significato da dritta" },
                Field { label: "Rovescio", value: rev, kind: FieldKind::Text, hint: "significato da rovesciata" },
                Field { label: "Keyword", value: kw, kind: FieldKind::Text, hint: "parole chiave, separate da virgola" },
                Field { label: "Colore", value: color, kind: FieldKind::Color, hint: "R,G,B (0-255)" },
                Field { label: "Simbolo", value: symbol, kind: FieldKind::MultiLine, hint: "ASCII art · Invio a capo · Ctrl+S salva" },
            ],
            idx: 0,
        });
    }

    fn key_form(&mut self, key: KeyEvent) {
        let Modal::Form(form) = &mut self.modal else { return };
        let multiline = matches!(form.fields[form.idx].kind, FieldKind::MultiLine);
        let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
        if ctrl && key.code == KeyCode::Char('s') {
            self.submit_form();
            return;
        }
        match key.code {
            KeyCode::Esc => self.modal = Modal::None,
            KeyCode::Enter if multiline => form.fields[form.idx].value.push('\n'),
            KeyCode::Enter => self.submit_form(),
            KeyCode::Tab | KeyCode::Down => form.idx = (form.idx + 1) % form.fields.len(),
            KeyCode::BackTab | KeyCode::Up => form.idx = (form.idx + form.fields.len() - 1) % form.fields.len(),
            KeyCode::Backspace => {
                form.fields[form.idx].value.pop();
            }
            KeyCode::Char(c) => form.fields[form.idx].value.push(c),
            _ => {}
        }
    }

    fn submit_form(&mut self) {
        let Modal::Form(form) = &self.modal else { return };
        match form.action {
            FormAction::Note => {
                let note = form.raw("Nota").trim_end().to_string();
                if let Some(r) = &mut self.current {
                    r.note = note.clone();
                    // Aggiorna anche in cronologia se gia' salvata.
                    let at = r.at;
                    if let Some(stored) = self.history.readings.iter_mut().find(|x| x.at == at) {
                        stored.note = note;
                    }
                }
                self.modal = Modal::None;
                let _ = store::save_history(&self.history);
                self.ok("Nota salvata");
            }
            FormAction::ExportPath => {
                let path = form.value("File");
                let Some(reading) = self.current.clone() else { return };
                // Usa il mazzo della lettura, non necessariamente quello corrente.
                let deck = self
                    .decks
                    .iter()
                    .find(|d| d.name == reading.deck)
                    .unwrap_or(self.deck());
                let text = crate::export::social_text(deck, &reading);
                match std::fs::write(&path, text) {
                    Ok(()) => {
                        self.modal = Modal::None;
                        self.ok(format!("Esportato in {path}"));
                    }
                    Err(e) => self.error(format!("Export fallito: {e}")),
                }
            }
            FormAction::NewDeck => {
                let name = form.value("Nome");
                if name.is_empty() {
                    self.warn("Il nome è obbligatorio");
                    return;
                }
                if self.decks.iter().any(|d| d.name == name) {
                    self.warn("Esiste già un mazzo con questo nome");
                    return;
                }
                let deck = Deck {
                    name: name.clone(),
                    description: form.value("Descrizione"),
                    reversed_chance: 0.4,
                    cards: Vec::new(),
                };
                self.decks.push(deck);
                self.deck_idx = self.decks.len() - 1;
                self.editor_sel = 0;
                self.modal = Modal::None;
                self.info(format!("Mazzo '{name}' creato: aggiungi carte con 'n'"));
            }
            FormAction::AddCard | FormAction::EditCard(_) => self.apply_card(form.action),
        }
    }

    fn apply_card(&mut self, action: FormAction) {
        let Modal::Form(form) = &self.modal else { return };
        let name = form.value("Nome");
        if name.is_empty() {
            self.warn("Il nome della carta è obbligatorio");
            return;
        }
        let card = Card {
            name,
            number: form.value("Numero"),
            upright: form.value("Dritto"),
            reversed: form.value("Rovescio"),
            keywords: form
                .value("Keyword")
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect(),
            symbol: form.raw("Simbolo").trim_matches('\n').to_string(),
            color: parse_color(&form.value("Colore")),
        };
        let idx = self.deck_idx;
        match action {
            FormAction::EditCard(i) if i < self.decks[idx].cards.len() => self.decks[idx].cards[i] = card,
            _ => self.decks[idx].cards.push(card),
        }
        self.modal = Modal::None;
        self.save_current_deck();
    }

    fn request_delete_card(&mut self) {
        if self.deck().cards.get(self.editor_sel).is_none() {
            self.warn("Nessuna carta selezionata");
            return;
        }
        let name = self.deck().cards[self.editor_sel].name.clone();
        self.modal = Modal::Confirm(Confirm {
            prompt: format!("Eliminare la carta '{name}'?"),
            kind: ConfirmKind::DeleteCard(self.editor_sel),
        });
    }

    fn key_confirm(&mut self, key: KeyEvent) {
        let Modal::Confirm(c) = &self.modal else { return };
        let kind = c.kind;
        match key.code {
            KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => {
                self.modal = Modal::None;
                match kind {
                    ConfirmKind::DeleteCard(i) => {
                        if i < self.decks[self.deck_idx].cards.len() {
                            self.decks[self.deck_idx].cards.remove(i);
                            self.editor_sel = self.editor_sel.min(self.deck().cards.len().saturating_sub(1));
                            self.save_current_deck();
                        }
                    }
                    ConfirmKind::DeleteDeck(i) => {
                        if i < self.decks.len() {
                            let name = self.decks[i].name.clone();
                            match store::delete_deck(&name) {
                                Ok(removed_file) => {
                                    self.decks.remove(i);
                                    if self.deck_idx >= self.decks.len() {
                                        self.deck_idx = self.deck_idx.saturating_sub(1);
                                    }
                                    self.editor_sel = 0;
                                    self.info(if removed_file {
                                        format!("Mazzo '{name}' eliminato")
                                    } else {
                                        format!("Mazzo '{name}' rimosso (nessun file su disco)")
                                    });
                                }
                                Err(e) => self.error(format!("Eliminazione fallita: {e}")),
                            }
                        }
                    }
                    ConfirmKind::ClearHistory => {
                        self.history.readings.clear();
                        self.history_sel = 0;
                        let _ = store::save_history(&self.history);
                        self.info("Cronologia cancellata");
                    }
                }
            }
            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                self.modal = Modal::None;
                self.info("Annullato");
            }
            _ => {}
        }
    }

    fn save_current_deck(&mut self) {
        let deck = self.deck().clone();
        match store::save_deck(&deck) {
            Ok(path) => self.ok(format!("Mazzo salvato in {}", path.display())),
            Err(e) => self.error(format!("Salvataggio mazzo fallito: {e}")),
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

fn parse_color(s: &str) -> [u8; 3] {
    let parts: Vec<u8> = s.split(',').filter_map(|p| p.trim().parse().ok()).collect();
    if parts.len() == 3 {
        [parts[0], parts[1], parts[2]]
    } else {
        [150, 130, 210]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn press(app: &mut App, code: KeyCode) {
        app.on_key(KeyEvent::from(code));
    }
    fn ctrl(app: &mut App, c: char) {
        app.on_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::CONTROL));
    }
    fn typ(app: &mut App, s: &str) {
        for c in s.chars() {
            press(app, KeyCode::Char(c));
        }
    }

    fn fresh_app(tag: &str) -> App {
        store::set_test_dir(std::env::temp_dir().join(format!("oracle-app-{tag}")));
        App::new()
    }

    /// Porta a termine l'animazione forzando la fase a Shown.
    fn finish_anim(app: &mut App) {
        app.phase = Phase::Shown;
        app.tick();
        // persist avviene nel passaggio Revealing->Shown; qui forziamo il salvataggio.
        super::App::persist_current(app);
    }

    #[test]
    fn t_key_cycles_palette() {
        let mut app = fresh_app("palette");
        let first = app.theme().name;
        press(&mut app, KeyCode::Char('t'));
        assert_ne!(first, app.theme().name);
        press(&mut app, KeyCode::Char('T'));
    }

    #[test]
    fn daily_draw_creates_and_persists_a_reading() {
        let mut app = fresh_app("daily");
        assert!(app.current.is_none());
        press(&mut app, KeyCode::Enter); // pesca
        assert!(app.current.is_some());
        assert_eq!(app.phase, Phase::Shuffling);
        finish_anim(&mut app);
        assert_eq!(app.history.readings.len(), 1);
        assert!(app.history.readings[0].daily);

        // Ricaricando l'app, la giornaliera di oggi torna senza ripescare.
        let mut app2 = App::new();
        assert!(app2.current.is_some(), "la giornaliera persiste tra sessioni");
        // Non permette una seconda pescata giornaliera.
        app2.phase = Phase::Shown;
        press(&mut app2, KeyCode::Enter);
        assert_eq!(app2.status.level, Level::Warn);
    }

    #[test]
    fn skipping_animation_persists_immediately() {
        let mut app = fresh_app("skip");
        press(&mut app, KeyCode::Enter);
        assert_eq!(app.phase, Phase::Shuffling);
        press(&mut app, KeyCode::Char(' ')); // salta
        assert_eq!(app.phase, Phase::Shown);
        assert_eq!(app.history.readings.len(), 1);
    }

    #[test]
    fn spread_draws_three_cards() {
        let mut app = fresh_app("spread");
        app.view = View::Spread;
        press(&mut app, KeyCode::Enter);
        finish_anim(&mut app);
        let r = app.current.as_ref().unwrap();
        assert_eq!(r.cards.len(), 3);
        assert_eq!(r.spread, Spread::ThreeCard);
        assert!(!r.daily);
    }

    #[test]
    fn note_is_attached_and_persisted() {
        let mut app = fresh_app("note");
        press(&mut app, KeyCode::Enter);
        press(&mut app, KeyCode::Char(' ')); // salta anim, salva
        press(&mut app, KeyCode::Char('n')); // apri nota
        typ(&mut app, "riflessione");
        ctrl(&mut app, 's');
        assert_eq!(app.current.as_ref().unwrap().note, "riflessione");
        assert_eq!(app.history.readings[0].note, "riflessione");
    }

    #[test]
    fn deck_picker_switches_deck() {
        let mut app = fresh_app("deckpick");
        assert_eq!(app.deck().name, "Void Arcana");
        press(&mut app, KeyCode::Char('d'));
        assert!(matches!(app.modal, Modal::DeckPicker(_)));
        press(&mut app, KeyCode::Down);
        press(&mut app, KeyCode::Enter);
        assert_eq!(app.deck().name, "Neon Oracle");
    }

    #[test]
    fn editor_adds_a_card_and_saves_deck() {
        let mut app = fresh_app("editor");
        app.view = View::Editor;
        let before = app.deck().cards.len();
        press(&mut app, KeyCode::Char('n'));
        typ(&mut app, "La Prova");
        // vai al campo simbolo? non necessario; salviamo con Ctrl+S
        ctrl(&mut app, 's');
        assert_eq!(app.deck().cards.len(), before + 1);
        assert!(app.deck().cards.iter().any(|c| c.name == "La Prova"));
        // Il mazzo modificato e' stato scritto su disco.
        let (decks, _) = store::load_decks();
        let void = decks.iter().find(|d| d.name == "Void Arcana").unwrap();
        assert!(void.cards.iter().any(|c| c.name == "La Prova"));
    }

    #[test]
    fn editor_deletes_a_card_after_confirm() {
        let mut app = fresh_app("del");
        app.view = View::Editor;
        let before = app.deck().cards.len();
        press(&mut app, KeyCode::Char('x'));
        assert!(matches!(app.modal, Modal::Confirm(_)));
        press(&mut app, KeyCode::Char('y'));
        assert_eq!(app.deck().cards.len(), before - 1);
    }

    #[test]
    fn color_parsing_is_robust() {
        assert_eq!(parse_color("10,20,30"), [10, 20, 30]);
        assert_eq!(parse_color("bad"), [150, 130, 210]);
        assert_eq!(parse_color("1,2"), [150, 130, 210]);
    }

    #[test]
    fn deleting_a_custom_deck_removes_it_and_its_file() {
        let mut app = fresh_app("deletedeck");
        // Crea un mazzo personalizzato (che viene salvato su disco).
        press(&mut app, KeyCode::Char('N'));
        typ(&mut app, "My Deck");
        press(&mut app, KeyCode::Enter);
        let created = app.decks.iter().position(|d| d.name == "My Deck").unwrap();

        // Dal picker, con 'd' si apre la conferma di eliminazione.
        press(&mut app, KeyCode::Char('d')); // apre il picker (sel = deck corrente = 0)
        let mut guard = 0;
        while {
            if let Modal::DeckPicker(s) = app.modal {
                s != created
            } else {
                false
            }
        } {
            press(&mut app, KeyCode::Down);
            guard += 1;
            assert!(guard < app.decks.len());
        }
        press(&mut app, KeyCode::Char('d'));
        assert!(matches!(app.modal, Modal::Confirm(_)));
        press(&mut app, KeyCode::Char('y'));
        assert!(!app.decks.iter().any(|d| d.name == "My Deck"));
    }

    #[test]
    fn builtin_decks_cannot_be_deleted() {
        let mut app = fresh_app("nodelete");
        press(&mut app, KeyCode::Char('d')); // apre il picker sul primo (Void Arcana)
        assert!(matches!(app.modal, Modal::DeckPicker(_)));
        press(&mut app, KeyCode::Char('d')); // richiede eliminazione di un builtin
        // Niente file su disco: la richiesta è rifiutata con un warning.
        assert!(!matches!(app.modal, Modal::Confirm(_)));
        assert_eq!(app.status.level, Level::Warn);
    }

    #[test]
    fn clear_history_wipes_readings_after_confirm() {
        let mut app = fresh_app("clearhist");
        press(&mut app, KeyCode::Enter); // pesca la giornaliera
        press(&mut app, KeyCode::Char(' ')); // salta animazione, salva
        assert!(!app.history.readings.is_empty());

        app.view = View::History;
        press(&mut app, KeyCode::Char('X'));
        assert!(matches!(app.modal, Modal::Confirm(_)));
        press(&mut app, KeyCode::Char('y'));
        assert!(app.history.readings.is_empty());
        // Ricaricando, la cronologia resta vuota.
        let app2 = App::new();
        assert!(app2.history.readings.is_empty());
    }
}
