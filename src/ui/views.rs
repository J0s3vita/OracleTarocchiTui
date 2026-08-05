//! Le viste principali: giornaliera, stesura a tre carte, cronologia, editor, aiuto.

use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, List, ListItem, ListState, Paragraph, Wrap};
use ratatui::Frame;

use crate::app::{App, Phase};
use crate::model::{Reading, Spread};
use crate::theme::Theme;
use crate::ui::card;

// ------------------------------------------------------------------- Giornaliera

pub fn daily(frame: &mut Frame, area: Rect, app: &App) {
    let theme = app.theme();
    match &app.current {
        Some(reading) if reading.spread == Spread::Single => {
            let rows = Layout::vertical([Constraint::Min(8), Constraint::Length(7)]).split(area);
            let cols = Layout::horizontal([Constraint::Percentage(50)])
                .flex(ratatui::layout::Flex::Center)
                .split(rows[0]);
            render_single_card(frame, cols[0], app, reading, &theme);
            render_reading_meaning(frame, rows[1], app, reading, &theme);
        }
        _ => render_draw_invite(frame, area, app, &theme, "la carta del giorno"),
    }
}

fn render_draw_invite(frame: &mut Frame, area: Rect, app: &App, theme: &Theme, what: &str) {
    let inner = crate::ui::panel(frame, area, theme, " Oracolo ", true);
    let msg = if matches!(app.phase, Phase::Shuffling | Phase::Revealing) {
        "mescolando il velo..."
    } else {
        "Premi Invio per pescare"
    };
    frame.render_widget(
        Paragraph::new(vec![
            Line::from(""),
            Line::from(Span::styled(
                format!("✦ {} ✦", app.deck().name),
                Style::default().fg(theme.accent).add_modifier(Modifier::BOLD),
            ))
            .alignment(Alignment::Center),
            Line::from(""),
            Line::from(Span::styled(format!("Pesca {what}.", ), theme.dimmed())).alignment(Alignment::Center),
            Line::from(""),
            Line::from(Span::styled(msg, Style::default().fg(theme.accent2))).alignment(Alignment::Center),
        ]),
        inner,
    );
}

/// Se e' in corso un'animazione mostra il dorso animato; altrimenti la carta.
fn render_single_card(frame: &mut Frame, area: Rect, app: &App, reading: &Reading, theme: &Theme) {
    let drawn = &reading.cards[0];
    if let Some(c) = app.card_by_name(&drawn.card) {
        card::render(
            frame,
            area,
            c,
            drawn.orientation,
            app.phase,
            app.phase_progress(),
            app.elapsed(),
            "",
            theme,
        );
    }
}

fn render_reading_meaning(frame: &mut Frame, area: Rect, app: &App, reading: &Reading, theme: &Theme) {
    let inner = crate::ui::panel(frame, area, theme, " Significato ", false);

    if !matches!(app.phase, Phase::Shown) {
        frame.render_widget(
            Paragraph::new(Span::styled(" ...", theme.dimmed())),
            inner,
        );
        return;
    }

    let drawn = &reading.cards[0];
    if let Some(c) = app.card_by_name(&drawn.card) {
        let mut lines = card::meaning_lines(c, drawn.orientation, theme);
        if !reading.note.trim().is_empty() {
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                format!("« {} »", reading.note.trim()),
                Style::default().fg(theme.accent2).add_modifier(Modifier::ITALIC),
            )));
        }
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            " 'n' nota · 'e' esporta per social",
            theme.dimmed(),
        )));
        frame.render_widget(Paragraph::new(lines).wrap(Wrap { trim: false }), inner);
    }
}

// ------------------------------------------------------------------- Stesura

pub fn spread(frame: &mut Frame, area: Rect, app: &App) {
    let theme = app.theme();
    match &app.current {
        Some(reading) if reading.spread == Spread::ThreeCard => {
            let rows = Layout::vertical([Constraint::Min(9), Constraint::Length(8)]).split(area);
            let cols = Layout::horizontal([
                Constraint::Percentage(33),
                Constraint::Percentage(34),
                Constraint::Percentage(33),
            ])
            .split(rows[0]);
            let positions = Spread::ThreeCard.positions();
            for (i, drawn) in reading.cards.iter().enumerate() {
                if let Some(c) = app.card_by_name(&drawn.card) {
                    // Le tre carte si rivelano in sequenza (sfasando il progresso).
                    let staggered = stagger(app.phase_progress(), i, reading.cards.len());
                    card::render(
                        frame,
                        cols[i],
                        c,
                        drawn.orientation,
                        app.phase,
                        staggered,
                        app.elapsed(),
                        positions.get(i).copied().unwrap_or(""),
                        &theme,
                    );
                }
            }
            render_spread_meaning(frame, rows[1], app, reading, &theme);
        }
        _ => render_draw_invite(frame, area, app, &theme, "una stesura Passato · Presente · Futuro"),
    }
}

/// Sfasa il progresso per rivelare le carte una dopo l'altra.
fn stagger(progress: f32, i: usize, n: usize) -> f32 {
    let slice = 1.0 / n as f32;
    ((progress - slice * i as f32) / slice).clamp(0.0, 1.0)
}

fn render_spread_meaning(frame: &mut Frame, area: Rect, app: &App, reading: &Reading, theme: &Theme) {
    let inner = crate::ui::panel(frame, area, theme, " Lettura ", false);

    if !matches!(app.phase, Phase::Shown) {
        frame.render_widget(Paragraph::new(Span::styled(" ...", theme.dimmed())), inner);
        return;
    }

    let positions = Spread::ThreeCard.positions();
    let mut lines = Vec::new();
    for (i, drawn) in reading.cards.iter().enumerate() {
        if let Some(c) = app.card_by_name(&drawn.card) {
            let pos = positions.get(i).copied().unwrap_or("");
            lines.push(Line::from(vec![
                Span::styled(format!(" {pos}: ", ), Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
                Span::styled(
                    format!("{}{}", c.name, if drawn.orientation.is_reversed() { " (rov.)" } else { "" }),
                    Style::default().fg(theme.fg).add_modifier(Modifier::BOLD),
                ),
                Span::styled(format!("  {}", c.meaning(drawn.orientation)), theme.dimmed()),
            ]));
        }
    }
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        " Invio ripesca · 'n' nota · 'e' esporta",
        theme.dimmed(),
    )));
    frame.render_widget(Paragraph::new(lines).wrap(Wrap { trim: false }), inner);
}

// ------------------------------------------------------------------- Cronologia
pub fn history(frame: &mut Frame, area: Rect, app: &App) {
    let theme = app.theme();
    let recent = app.history.recent();
    let title = format!(" Cronologia ({}) ", recent.len());
    let inner = crate::ui::panel(frame, area, &theme, &title, true);

    if recent.is_empty() {
        frame.render_widget(
            Paragraph::new(Span::styled(
                " Nessuna lettura ancora. Pesca una carta dalla vista Giornaliera.",
                theme.dimmed(),
            )),
            inner,
        );
        return;
    }

    let items: Vec<ListItem> = recent
        .iter()
        .map(|r| {
            let daily_mark = if r.daily {
                Span::styled("☾ ", Style::default().fg(theme.accent2))
            } else {
                Span::styled("  ", theme.dimmed())
            };
            let mut line1 = vec![
                daily_mark,
                Span::styled(
                    r.at.format("%d.%m.%Y %H:%M").to_string(),
                    Style::default().fg(theme.accent),
                ),
                Span::styled(format!("  {}", r.deck), theme.dimmed()),
            ];
            if !r.note.trim().is_empty() {
                line1.push(Span::styled("  ✎", Style::default().fg(theme.accent2)));
            }
            ListItem::new(vec![
                Line::from(line1),
                Line::from(Span::styled(format!("    {}", r.summary()), Style::default().fg(theme.fg))),
            ])
        })
        .collect();

    let mut state = ListState::default().with_selected(Some(app.history_sel.min(recent.len() - 1)));
    frame.render_stateful_widget(
        List::new(items)
            .block(Block::default())
            .highlight_style(theme.selection())
            .highlight_symbol("▶ "),
        inner,
        &mut state,
    );
}

// ------------------------------------------------------------------- Editor

pub fn editor(frame: &mut Frame, area: Rect, app: &App) {
    let theme = app.theme();
    let cols = Layout::horizontal([Constraint::Length(30), Constraint::Min(20)]).split(area);

    let title = format!(" {} ({}) ", app.deck().name, app.deck().cards.len());
    let inner = crate::ui::panel(frame, cols[0], &theme, &title, true);

    if app.deck().cards.is_empty() {
        frame.render_widget(
            Paragraph::new(vec![
                Line::from(""),
                Line::from(Span::styled(" Mazzo vuoto.", theme.dimmed())),
                Line::from(Span::styled(" 'n' nuova carta.", theme.dimmed())),
            ]),
            inner,
        );
    } else {
        let items: Vec<ListItem> = app
            .deck()
            .cards
            .iter()
            .map(|c| {
                ListItem::new(Line::from(vec![
                    Span::styled(
                        format!("{:<4}", c.number),
                        Style::default().fg(crate::theme::card_color(c.color)),
                    ),
                    Span::styled(c.name.clone(), Style::default().fg(theme.fg)),
                ]))
            })
            .collect();
        let mut state = ListState::default().with_selected(Some(app.editor_sel.min(app.deck().cards.len() - 1)));
        frame.render_stateful_widget(
            List::new(items)
                .block(Block::default())
                .highlight_style(theme.selection())
                .highlight_symbol("▶ "),
            inner,
            &mut state,
        );
    }

    render_card_preview(frame, cols[1], app, &theme);
}

fn render_card_preview(frame: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let inner = crate::ui::panel(frame, area, theme, " Anteprima ", false);

    let Some(c) = app.deck().cards.get(app.editor_sel) else {
        frame.render_widget(
            Paragraph::new(Span::styled(
                " 'n' nuova · 'e' modifica · 'x' elimina · 'N' nuovo mazzo · 's' salva",
                theme.dimmed(),
            )),
            inner,
        );
        return;
    };

    let split = Layout::vertical([Constraint::Min(6), Constraint::Length(7)]).split(inner);
    // Simbolo statico (fase Shown) al centro.
    card::render(
        frame,
        split[0],
        c,
        crate::model::Orientation::Upright,
        Phase::Shown,
        1.0,
        0.0,
        &c.name,
        theme,
    );
    card::render_meaning(frame, split[1], c, crate::model::Orientation::Upright, theme);
}

// ------------------------------------------------------------------- Aiuto

pub fn help(frame: &mut Frame, area: Rect, app: &App) {
    let theme = app.theme();
    let cols = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(area);

    let ki = crate::ui::panel(frame, cols[0], &theme, " Tasti ", true);
    frame.render_widget(Paragraph::new(keys(&theme)), ki);

    let ii = crate::ui::panel(frame, cols[1], &theme, " Il mazzo ", false);
    frame.render_widget(Paragraph::new(about(app, &theme)).wrap(Wrap { trim: false }), ii);
}

fn kv<'a>(k: &'a str, d: &'a str, theme: &Theme) -> Line<'a> {
    Line::from(vec![
        Span::styled(format!("  {k:<11}"), Style::default().fg(theme.accent2)),
        Span::styled(d, Style::default().fg(theme.fg)),
    ])
}
fn sec<'a>(t: &'a str, theme: &Theme) -> Line<'a> {
    Line::from(Span::styled(format!(" {t}"), Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)))
}

fn keys<'a>(theme: &Theme) -> Vec<Line<'a>> {
    vec![
        sec("Globali", theme),
        kv("1-5 / Tab", "cambia vista", theme),
        kv("d", "scegli mazzo", theme),
        kv("t", "cambia palette", theme),
        kv("q", "esci", theme),
        Line::from(""),
        sec("Pescaggio", theme),
        kv("Invio", "pesca (giornaliera / stesura)", theme),
        kv("Spazio", "salta l'animazione", theme),
        kv("n", "aggiungi nota alla lettura", theme),
        kv("e", "esporta per social", theme),
        Line::from(""),
        sec("Cronologia", theme),
        kv("↑ ↓ / j k", "scorri", theme),
        kv("PagSu/PagGiù", "salta di 10", theme),
        kv("Home / End", "inizio / fine lista", theme),
        kv("Invio", "riapri lettura", theme),
        kv("e", "esporta la lettura scelta", theme),
        kv("X", "cancella tutta la cronologia", theme),
        Line::from(""),
        sec("Editor mazzo", theme),
        kv("n / e / x", "nuova / modifica / elimina carta", theme),
        kv("N", "nuovo mazzo", theme),
        kv("s", "salva mazzo su disco", theme),
        Line::from(""),
        sec("Scegliere & mazzi", theme),
        kv("d", "apri la scelta mazzo", theme),
        kv("d", "nel picker: elimina il mazzo scelto", theme),
        kv("N", "crea un mazzo (da ovunque)", theme),
        kv("Ctrl+S", "salva form multilinea", theme),
    ]
}

fn about<'a>(app: &App, theme: &Theme) -> Vec<Line<'a>> {
    let p = |s: String, c| Line::from(Span::styled(format!("  {s}"), Style::default().fg(c)));
    let deck = app.deck();
    vec![
        sec("Mazzo attivo", theme),
        p(format!("{} — {} carte", deck.name, deck.cards.len()), theme.fg),
        p(deck.description.clone(), theme.dim),
        p(format!("probabilità rovesciata: {:.0}%", deck.reversed_chance * 100.0), theme.dim),
        Line::from(""),
        sec("Mazzi disponibili", theme),
        p(app.decks.iter().map(|d| d.name.clone()).collect::<Vec<_>>().join(", "), theme.accent2),
        Line::from(""),
        sec("File", theme),
        Line::from(vec![
            Span::styled("  cartella   ", theme.dimmed()),
            Span::styled(crate::store::data_dir().display().to_string(), Style::default().fg(theme.accent2)),
        ]),
        p("mazzi in decks/*.toml · cronologia in history.json".into(), theme.dim),
        p("override: variabile d'ambiente ORACLE_DIR".into(), theme.dim),
        Line::from(""),
        sec("Note", theme),
        p("La carta del giorno si pesca una volta sola: torna domani.".into(), theme.dim),
        p("Ogni carta può uscire rovesciata, con un significato diverso.".into(), theme.dim),
    ]
}
