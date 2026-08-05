//! Composizione: sfondo particellare, schede, viste, barra di stato, modali.

pub mod card;
pub mod particles;
pub mod views;

use ratatui::layout::{Alignment, Constraint, Flex, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph, Tabs, Wrap};
use ratatui::Frame;

use crate::app::{App, FieldKind, Level, Modal, View};
use crate::theme::Theme;

pub fn render(frame: &mut Frame, app: &App) {
    let theme = app.theme();
    let area = frame.area();
    frame.render_widget(Block::default().style(theme.base()), area);

    // Sfondo particellare, dietro a tutto.
    particles::render(frame, area, app.elapsed(), &theme);

    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(6),
        Constraint::Length(1),
    ])
    .split(area);

    render_header(frame, chunks[0], app, &theme);
    match app.view {
        View::Daily => views::daily(frame, chunks[1], app),
        View::Spread => views::spread(frame, chunks[1], app),
        View::History => views::history(frame, chunks[1], app),
        View::Editor => views::editor(frame, chunks[1], app),
        View::Help => views::help(frame, chunks[1], app),
    }
    render_footer(frame, chunks[2], app, &theme);

    match &app.modal {
        Modal::DeckPicker(sel) => render_deck_picker(frame, area, app, &theme, *sel),
        Modal::Form(_) => render_form(frame, area, app, &theme),
        Modal::Confirm(_) => render_confirm(frame, area, app, &theme),
        Modal::None => {}
    }
}

fn render_header(frame: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let titles: Vec<Line> = View::TABS
        .iter()
        .enumerate()
        .map(|(i, v)| Line::from(vec![Span::styled(format!("{} ", i + 1), theme.dimmed()), Span::raw(v.label())]))
        .collect();
    let selected = View::TABS.iter().position(|v| *v == app.view).unwrap_or(0);
    let today = chrono::Local::now().format("%d.%m.%Y").to_string();
    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(theme.border))
                .title(Line::from(vec![
                    Span::styled(format!(" ☾ ORACLE · {} ", theme.name), theme.title()),
                    Span::styled(format!("  {today} "), Style::default().fg(theme.dim)),
                ])),
        )
        .style(Style::default().fg(theme.dim))
        .highlight_style(Style::default().fg(theme.accent).add_modifier(Modifier::BOLD | Modifier::REVERSED))
        .select(selected)
        .divider(Span::styled(" · ", Style::default().fg(theme.border)));
    frame.render_widget(tabs, area);
}

fn render_footer(frame: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let (msg, style) = if app.status.is_stale() {
        (hint(app), theme.dimmed())
    } else {
        let color = match app.status.level {
            Level::Info => theme.fg,
            Level::Ok => theme.ok,
            Level::Warn => theme.warn,
            Level::Error => theme.danger,
        };
        (app.status.text.clone(), Style::default().fg(color))
    };
    let cols = Layout::horizontal([Constraint::Min(20), Constraint::Length(24)]).split(area);
    frame.render_widget(Paragraph::new(Span::styled(format!(" {msg}"), style)), cols[0]);
    frame.render_widget(
        Paragraph::new(Span::styled("d mazzo · t palette · q esci ", theme.dimmed())).alignment(Alignment::Right),
        cols[1],
    );
}

fn hint(app: &App) -> String {
    match app.view {
        View::Daily => "Invio pesca · n nota · e esporta · d mazzo".into(),
        View::Spread => "Invio pesca 3 carte · n nota · e esporta".into(),
        View::History => "↑↓ scorri · Invio riapri · e esporta · X cancella tutto".into(),
        View::Editor => "n/e/x carta · N nuovo mazzo · s salva".into(),
        View::Help => "Tab per tornare alle viste".into(),
    }
}

pub fn block<'a>(theme: &Theme, title: &'a str, focused: bool) -> Block<'a> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        // Sfondo pieno: copre le particelle dentro il pannello, lasciandole solo
        // nei margini esterni.
        .style(Style::default().bg(theme.bg))
        .border_style(theme.border_style(focused))
        .title(Span::styled(
            title.to_string(),
            if focused { theme.title() } else { Style::default().fg(theme.dim) },
        ))
}

/// Disegna un pannello con lo sfondo pulito (cancella le particelle sotto) e
/// restituisce l'area interna. Da usare al posto di `block` + render manuale.
pub fn panel(frame: &mut Frame, area: Rect, theme: &Theme, title: &str, focused: bool) -> Rect {
    let b = block(theme, title, focused);
    let inner = b.inner(area);
    frame.render_widget(Clear, area);
    frame.render_widget(b, area);
    inner
}

fn centered(area: Rect, w: u16, h: u16) -> Rect {
    let horizontal = Layout::horizontal([Constraint::Length(w.min(area.width))]).flex(Flex::Center).split(area);
    Layout::vertical([Constraint::Length(h.min(area.height))]).flex(Flex::Center).split(horizontal[0])[0]
}

fn modal_block<'a>(theme: &Theme, title: &'a str, color: ratatui::style::Color) -> Block<'a> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(color))
        .style(Style::default().bg(theme.bg))
        .title(Span::styled(title.to_string(), Style::default().fg(color).add_modifier(Modifier::BOLD)))
}

fn render_deck_picker(frame: &mut Frame, area: Rect, app: &App, theme: &Theme, sel: usize) {
    let h = (app.decks.len() as u16 + 4).min(area.height.saturating_sub(2)).max(5);
    let popup = centered(area, 56, h);
    frame.render_widget(Clear, popup);
    let b = modal_block(theme, " Scegli mazzo ", theme.accent);
    let inner = b.inner(popup);
    frame.render_widget(b, popup);

    let mut lines = Vec::new();
    for (i, deck) in app.decks.iter().enumerate() {
        let selected = i == sel;
        let marker = if selected { "▶ " } else { "  " };
        let style = if selected {
            Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.fg)
        };
        lines.push(Line::from(vec![
            Span::styled(marker, style),
            Span::styled(format!("{:<16}", deck.name), style),
            Span::styled(
                if app.builtin_decks.iter().any(|b| b == &deck.name) { "★" } else { "✎" },
                theme.dimmed(),
            ),
            Span::styled(format!(" {} carte", deck.cards.len()), theme.dimmed()),
        ]));
    }
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        " Invio sceglie · d elimina · Esc annulla",
        theme.dimmed(),
    )));
    frame.render_widget(Paragraph::new(lines), inner);
}

fn render_form(frame: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let Modal::Form(form) = &app.modal else { return };
    let rows: u16 = form
        .fields
        .iter()
        .map(|f| if matches!(f.kind, FieldKind::MultiLine) { 8 } else { 2 })
        .sum();
    let popup = centered(area, 70, (rows + 5).min(area.height.saturating_sub(2)));
    frame.render_widget(Clear, popup);
    let title = format!(" {} ", form.title);
    let b = modal_block(theme, &title, theme.accent);
    let inner = b.inner(popup);
    frame.render_widget(b, popup);

    let mut lines: Vec<Line> = Vec::new();
    for (i, field) in form.fields.iter().enumerate() {
        let active = i == form.idx;
        let marker = if active { "▶ " } else { "  " };
        let label_style = if active {
            Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)
        } else {
            theme.dimmed()
        };
        match field.kind {
            FieldKind::MultiLine => {
                lines.push(Line::from(vec![
                    Span::styled(marker, label_style),
                    Span::styled(format!("{} ", field.label), label_style),
                    Span::styled(field.hint, theme.dimmed()),
                ]));
                let shown: Vec<&str> = field.value.lines().collect();
                let start = shown.len().saturating_sub(6);
                for l in &shown[start..] {
                    lines.push(Line::from(Span::styled(format!("    {l}"), Style::default().fg(theme.fg))));
                }
                if active {
                    lines.push(Line::from(Span::styled("    ▏", Style::default().fg(theme.accent))));
                }
            }
            _ => {
                lines.push(Line::from(vec![
                    Span::styled(marker, label_style),
                    Span::styled(format!("{:<12}", field.label), label_style),
                    Span::styled(
                        format!("{}{}", field.value, if active { "▏" } else { "" }),
                        Style::default().fg(theme.fg),
                    ),
                ]));
                lines.push(Line::from(vec![
                    Span::raw("              "),
                    Span::styled(field.hint, theme.dimmed()),
                ]));
            }
        }
    }
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled("  Invio conferma · Tab campo · Esc annulla", theme.dimmed())));
    frame.render_widget(Paragraph::new(lines).wrap(Wrap { trim: false }), inner);
}

fn render_confirm(frame: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let Modal::Confirm(confirm) = &app.modal else { return };
    let popup = centered(area, 60, 7);
    frame.render_widget(Clear, popup);
    let b = modal_block(theme, " Conferma ", theme.danger);
    let inner = b.inner(popup);
    frame.render_widget(b, popup);
    frame.render_widget(
        Paragraph::new(vec![
            Line::from(""),
            Line::from(Span::styled(confirm.prompt.clone(), Style::default().fg(theme.fg))),
            Line::from(""),
            Line::from(Span::styled("y confermo · n annulla", Style::default().fg(theme.warn))),
        ])
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true }),
        inner,
    );
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::{App, Phase, View};
    use crossterm::event::{KeyCode, KeyEvent};
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    fn demo_app(tag: &str) -> App {
        crate::store::set_test_dir(std::env::temp_dir().join(format!("oracle-uitest-{tag}")));
        let mut app = App::new();
        // Pesca una giornaliera e falla arrivare a Shown, deterministicamente.
        app.on_key(KeyEvent::from(KeyCode::Enter));
        app.on_key(KeyEvent::from(KeyCode::Char(' '))); // salta l'animazione
        app
    }

    fn screen(app: &App, w: u16, h: u16) -> String {
        let mut t = Terminal::new(TestBackend::new(w, h)).unwrap();
        t.draw(|f| render(f, app)).unwrap();
        t.backend()
            .buffer()
            .content()
            .chunks(w as usize)
            .map(|r| r.iter().map(|c| c.symbol()).collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn every_view_renders_with_header() {
        let mut app = demo_app("views");
        for view in View::TABS {
            app.view = view;
            let out = screen(&app, 100, 32);
            assert!(out.contains("ORACLE"), "header assente in {view:?}");
            assert!(out.contains(view.label()), "scheda {view:?} non etichettata");
        }
    }

    #[test]
    fn daily_shows_a_card_and_meaning() {
        let mut app = demo_app("daily");
        app.view = View::Daily;
        assert_eq!(app.phase, Phase::Shown);
        let out = screen(&app, 100, 32);
        // Il nome della carta pescata compare da qualche parte.
        let name = &app.current.as_ref().unwrap().cards[0].card;
        assert!(out.contains(name.as_str()), "manca la carta '{name}':\n{out}");
        assert!(out.contains("Significato"));
    }

    #[test]
    fn history_lists_the_drawn_reading() {
        let mut app = demo_app("hist");
        app.view = View::History;
        let out = screen(&app, 100, 32);
        assert!(out.contains("Cronologia"));
        assert!(out.contains("Void Arcana"), "manca la lettura salvata:\n{out}");
    }

    #[test]
    fn editor_shows_deck_cards() {
        let mut app = demo_app("editor");
        app.view = View::Editor;
        let out = screen(&app, 100, 32);
        assert!(out.contains("Void Arcana"));
        assert!(out.contains("The Null"), "manca l'elenco carte:\n{out}");
    }

    #[test]
    fn deck_picker_modal_renders() {
        let mut app = demo_app("picker");
        app.on_key(KeyEvent::from(KeyCode::Char('d')));
        let out = screen(&app, 100, 32);
        assert!(out.contains("Scegli mazzo"));
        assert!(out.contains("Neon Oracle"));
    }

    #[test]
    fn narrow_terminal_does_not_panic() {
        let mut app = demo_app("narrow");
        for view in View::TABS {
            app.view = view;
            for (w, h) in [(40u16, 14u16), (60, 20), (200, 50)] {
                let _ = screen(&app, w, h);
            }
        }
    }

    /// Dump per controllo a occhio.
    /// `cargo test dump_screens -- --ignored --nocapture`
    #[test]
    #[ignore = "utile a mano"]
    fn dump_screens() {
        let mut app = demo_app("dump");
        for view in View::TABS {
            app.view = view;
            if view == View::Spread {
                app.on_key(KeyEvent::from(KeyCode::Enter));
                app.on_key(KeyEvent::from(KeyCode::Char(' ')));
            }
            println!("\n===== {:?} =====\n{}", view, screen(&app, 100, 32));
        }
    }

    /// Dump dei nuovi mazzi per controllo a occhio.
    /// `cargo test dump_new_decks -- --ignored --nocapture`
    #[test]
    #[ignore = "utile a mano"]
    fn dump_new_decks() {
        use crate::deck;
        for deck in [deck::mors_initium(), deck::athanor(), deck::la_soglia()] {
            for c in deck.cards.iter().take(4) {
                let mut t = Terminal::new(TestBackend::new(60, 24)).unwrap();
                let theme = crate::theme::THEMES[0];
                t.draw(|f| {
                    card::render(
                        f,
                        Rect {
                            x: 10,
                            y: 2,
                            width: 40,
                            height: 16,
                        },
                        c,
                        crate::model::Orientation::Upright,
                        Phase::Shown,
                        1.0,
                        0.0,
                        &format!("{} {}", deck.name, c.name),
                        &theme,
                    );
                })
                .unwrap();
                let out = t
                    .backend()
                    .buffer()
                    .content()
                    .chunks(60)
                    .map(|r| r.iter().map(|c| c.symbol()).collect::<String>())
                    .collect::<Vec<_>>()
                    .join("\n");
                println!("\n===== {}/{} =====\n{}", deck.name, c.name, out);
            }
        }
    }
}