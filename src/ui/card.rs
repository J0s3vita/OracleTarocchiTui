//! Rendering di una carta: cornice tinta col colore della carta, simbolo ASCII
//! con reveal a glitch, shuffle animato e simbolo capovolto se rovesciata.

use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap};
use ratatui::Frame;

use crate::app::Phase;
use crate::model::{Card, Orientation};
use crate::theme::{card_color, Theme};

/// Glifi usati per il rumore durante shuffle e glitch.
const GLITCH: &[char] = &[
    '▚', '▞', '▙', '▟', '█', '▓', '▒', '░', '#', '@', '%', '&', '/', '\\', '|', '*', '+', '=', '?',
    '§', '¤', '◊', '∆', '◐', '◑', '✦', '✧',
];

fn glitch_char(seed: u64) -> char {
    GLITCH[(seed as usize) % GLITCH.len()]
}

/// Disegna la carta dentro `area`. `progress` guida shuffle/reveal (0..1).
/// `t` e' il tempo globale (per l'animazione dello shuffle).
#[allow(clippy::too_many_arguments)]
pub fn render(
    frame: &mut Frame,
    area: Rect,
    card: &Card,
    orientation: Orientation,
    phase: Phase,
    progress: f32,
    t: f32,
    position: &str,
    theme: &Theme,
) {
    let reversed = orientation.is_reversed();
    let base = card_color(card.color);

    // La cornice si tinge del colore della carta (piena a reveal finito).
    let border_intensity = match phase {
        Phase::Shown => 1.0,
        Phase::Revealing => 0.4 + 0.6 * progress,
        _ => 0.3,
    };
    let border_color = theme.fade(base, border_intensity);

    let title = if position.is_empty() {
        String::new()
    } else {
        format!(" {position} ")
    };
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        // Sfondo pieno: il simbolo della carta non deve avere particelle dentro.
        .style(Style::default().bg(theme.bg))
        .border_style(Style::default().fg(border_color))
        .title(Span::styled(title, Style::default().fg(border_color).add_modifier(Modifier::BOLD)));
    let inner = block.inner(area);
    // Cancella eventuali particelle di sfondo prima di disegnare la carta.
    frame.render_widget(Clear, area);
    frame.render_widget(block, area);

    let w = inner.width as usize;
    let h = inner.height as usize;
    if w < 4 || h < 5 {
        return;
    }
    let header_visible = matches!(phase, Phase::Shown) || (matches!(phase, Phase::Revealing) && progress > 0.6);
    let corner = theme.fade(base, if header_visible { 0.9 } else { 0.4 });

    let mut lines: Vec<Line> = Vec::new();

    // --- Intestazione: angoli decorati + numero al centro ---
    let number = if header_visible && !card.number.is_empty() {
        format!("✧ {} ✧", card.number)
    } else {
        "✦   ✦".to_string()
    };
    lines.push(edge_line(w, '◈', &number, corner));
    lines.push(Line::from(""));

    // --- Corpo: simbolo ASCII (capovolto se rovesciata) con reveal a glitch ---
    // Righe riservate a intestazione (2) e piede (banner nome + divisori = 3).
    let reserved = 2 + 3 + if reversed { 1 } else { 0 };
    let body_h = h.saturating_sub(reserved).max(1);
    let symbol = symbol_lines(card, reversed);
    let rows = symbol.len().max(1);
    let pad_top = body_h.saturating_sub(symbol.len()) / 2;
    for _ in 0..pad_top {
        lines.push(Line::from(""));
    }
    for (r, raw) in symbol.iter().enumerate() {
        let line = match phase {
            Phase::Shuffling => back_row(w.min(24), r, t, base, theme),
            Phase::Revealing => reveal_row(raw, r, rows, progress, base, t, theme),
            _ => Line::from(Span::styled(
                raw.to_string(),
                Style::default().fg(base).add_modifier(Modifier::BOLD),
            )),
        };
        lines.push(line.alignment(Alignment::Center));
    }
    // Riempie fino al piede.
    let used = lines.len();
    let foot_at = h.saturating_sub(if reversed { 4 } else { 3 });
    for _ in used..foot_at {
        lines.push(Line::from(""));
    }

    // --- Piede: divisore ornamentale + nome + eventuale "rovesciata" ---
    lines.push(divider_line(w, base, header_visible, theme));
    if header_visible {
        lines.push(
            Line::from(Span::styled(
                card.name.to_uppercase(),
                Style::default().fg(theme.glow(base, 0.35)).add_modifier(Modifier::BOLD),
            ))
            .alignment(Alignment::Center),
        );
    } else {
        lines.push(Line::from(Span::styled("· · ·", theme.dimmed())).alignment(Alignment::Center));
    }
    if reversed {
        lines.push(
            Line::from(Span::styled(
                "⟲ rovesciata",
                Style::default().fg(theme.warn).add_modifier(Modifier::ITALIC),
            ))
            .alignment(Alignment::Center),
        );
    }

    frame.render_widget(Paragraph::new(lines), inner);
}

/// Riga a larghezza piena con un glifo agli angoli e un testo centrato.
fn edge_line(w: usize, corner_glyph: char, center: &str, color: Color) -> Line<'static> {
    let mut grid: Vec<char> = vec![' '; w];
    if w >= 2 {
        grid[0] = corner_glyph;
        grid[w - 1] = corner_glyph;
    }
    let cs: Vec<char> = center.chars().collect();
    if cs.len() + 2 <= w {
        let start = (w - cs.len()) / 2;
        for (i, ch) in cs.iter().enumerate() {
            grid[start + i] = *ch;
        }
    }
    Line::from(Span::styled(
        grid.into_iter().collect::<String>(),
        Style::default().fg(color).add_modifier(Modifier::BOLD),
    ))
}

/// Divisore ornamentale del piede: `⋅──── ◆ ────⋅`.
fn divider_line(w: usize, base: Color, revealed: bool, theme: &Theme) -> Line<'static> {
    let color = if revealed { theme.fade(base, 0.85) } else { theme.fade(base, 0.4) };
    let mut grid: Vec<char> = vec!['─'; w];
    if w >= 2 {
        grid[0] = '⋅';
        grid[w - 1] = '⋅';
    }
    let mid = ['◆'];
    let c = w / 2;
    // Spazio attorno al rombo centrale.
    for (i, ch) in mid.iter().enumerate() {
        let pos = c + i;
        if pos < w {
            grid[pos] = *ch;
        }
    }
    if c >= 1 {
        grid[c - 1] = ' ';
    }
    if c + 1 < w {
        grid[c + 1] = ' ';
    }
    Line::from(Span::styled(
        grid.into_iter().collect::<String>(),
        Style::default().fg(color),
    ))
}

/// Righe del simbolo, ripulite dall'indentazione comune (per un centraggio
/// pulito) e capovolte se la carta e' rovesciata.
fn symbol_lines(card: &Card, reversed: bool) -> Vec<String> {
    let raw: Vec<String> = card.symbol_lines().into_iter().map(|s| s.trim_end().to_string()).collect();
    // Toglie l'indentazione minima comune, cosi' il centraggio non sbanda.
    let min_indent = raw
        .iter()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.len() - l.trim_start().len())
        .min()
        .unwrap_or(0);
    let mut lines: Vec<String> = raw
        .into_iter()
        .map(|l| if l.len() >= min_indent { l[min_indent..].to_string() } else { l })
        .collect();
    if reversed {
        lines.reverse();
        for l in &mut lines {
            *l = l.chars().rev().map(flip_char).collect();
        }
    }
    lines
}

/// Specchia alcuni caratteri per rendere credibile il ribaltamento.
fn flip_char(c: char) -> char {
    match c {
        '/' => '\\',
        '\\' => '/',
        '(' => ')',
        ')' => '(',
        '<' => '>',
        '>' => '<',
        '╱' => '╲',
        '╲' => '╱',
        '▛' => '▜',
        '▜' => '▛',
        '▙' => '▟',
        '▟' => '▙',
        '◐' => '◑',
        '◑' => '◐',
        other => other,
    }
}

/// Riga durante lo shuffle: dorso della carta a reticolo diagonale che scorre.
fn back_row(width: usize, row: usize, t: f32, base: Color, theme: &Theme) -> Line<'static> {
    // Motivo a rombi/diagonali che scivola nel tempo: un "dorso" ornamentale.
    const MOTIF: [char; 4] = ['╱', '◇', '╲', '◈'];
    let shift = (t * 8.0) as usize;
    let mut spans: Vec<Span> = Vec::new();
    for col in 0..width {
        let k = (row + col + shift) % 4;
        let ch = MOTIF[k];
        // Alterna due intensita' per dare profondita' al reticolo.
        let bright = (row + col).is_multiple_of(2);
        let color = theme.fade(base, if bright { 0.55 } else { 0.3 });
        spans.push(Span::styled(ch.to_string(), Style::default().fg(color)));
    }
    Line::from(spans)
}

/// Riga durante il reveal: si stabilizza dall'alto verso il basso, i caratteri
/// non ancora "risolti" restano glitch.
fn reveal_row(raw: &str, row: usize, rows: usize, progress: f32, base: Color, t: f32, theme: &Theme) -> Line<'static> {
    // Soglia di risoluzione della riga in base alla posizione verticale.
    let row_threshold = row as f32 / rows as f32;
    let resolved = progress > row_threshold;
    let local = ((progress - row_threshold) / (1.0 / rows as f32)).clamp(0.0, 1.0);

    let mut spans: Vec<Span> = Vec::new();
    for (i, ch) in raw.chars().enumerate() {
        // Ogni carattere si risolve con una piccola probabilita' crescente.
        let noise = ((t * 25.0) as u64).wrapping_add((row as u64) * 5 + i as u64 * 11);
        let settled = resolved && (local > 0.5 || (noise % 100) as f32 / 100.0 < local);
        if ch == ' ' {
            spans.push(Span::raw(" "));
        } else if settled {
            spans.push(Span::styled(
                ch.to_string(),
                Style::default().fg(theme.glow(base, (local - 0.5).max(0.0))).add_modifier(Modifier::BOLD),
            ));
        } else {
            spans.push(Span::styled(
                glitch_char(noise).to_string(),
                Style::default().fg(theme.fade(base, 0.6)),
            ));
        }
    }
    Line::from(spans)
}

/// Testo del significato, mostrato sotto la carta.
pub fn meaning_lines<'a>(card: &'a Card, orientation: Orientation, theme: &Theme) -> Vec<Line<'a>> {
    let base = card_color(card.color);
    let mut lines = vec![Line::from(vec![
        Span::styled(
            format!("{} ", card.name),
            Style::default().fg(theme.glow(base, 0.2)).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("· {}", orientation.label()),
            Style::default().fg(if orientation.is_reversed() { theme.warn } else { theme.dim }),
        ),
    ])];
    if !card.keywords.is_empty() {
        lines.push(Line::from(Span::styled(
            card.keywords.join(" · "),
            Style::default().fg(theme.fade(base, 0.8)),
        )));
    }
    lines.push(Line::from(""));
    for l in wrap(card.meaning(orientation), 60) {
        lines.push(Line::from(Span::styled(l, Style::default().fg(theme.fg))));
    }
    lines
}

/// Wrapping semplice a parole.
fn wrap(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut cur = String::new();
    for word in text.split_whitespace() {
        if !cur.is_empty() && cur.chars().count() + 1 + word.chars().count() > width {
            lines.push(std::mem::take(&mut cur));
        }
        if !cur.is_empty() {
            cur.push(' ');
        }
        cur.push_str(word);
    }
    if !cur.is_empty() {
        lines.push(cur);
    }
    lines
}

/// Renderizza il testo di significato in un'area (usato dalle viste).
pub fn render_meaning(frame: &mut Frame, area: Rect, card: &Card, orientation: Orientation, theme: &Theme) {
    frame.render_widget(
        Paragraph::new(meaning_lines(card, orientation, theme)).wrap(Wrap { trim: false }),
        area,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn card() -> Card {
        Card {
            name: "Test".into(),
            number: "0".into(),
            upright: "una parola due tre quattro cinque sei sette otto nove dieci undici".into(),
            reversed: "rov".into(),
            keywords: vec!["a".into(), "b".into()],
            symbol: "/(o)\\\n ||| ".into(),
            color: [100, 120, 200],
        }
    }

    #[test]
    fn reversed_flips_symbol_lines() {
        let c = card();
        let up = symbol_lines(&c, false);
        let rev = symbol_lines(&c, true);
        // Prima riga capovolta diventa l'ultima, e i caratteri sono specchiati.
        assert_eq!(up.len(), rev.len());
        assert_ne!(up[0], rev[0]);
        // '/' -> '\' nello specchiamento.
        assert!(rev.iter().any(|l| l.contains('\\')));
    }

    #[test]
    fn flip_char_mirrors_slashes_and_parens() {
        assert_eq!(flip_char('/'), '\\');
        assert_eq!(flip_char('('), ')');
        assert_eq!(flip_char('╱'), '╲');
        assert_eq!(flip_char('x'), 'x');
    }

    #[test]
    fn wrap_respects_width() {
        let out = wrap("una parola due tre quattro cinque sei sette otto nove dieci undici", 20);
        assert!(out.len() > 1);
        assert!(out.iter().all(|l| l.chars().count() <= 20));
    }
}
