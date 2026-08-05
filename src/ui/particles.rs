//! Sfondo particellare lento: glifi che scendono piano, per un'atmosfera rituale.

use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::theme::Theme;

const GLYPHS: &[char] = &['·', '˙', '•', '∙', '⋅', '✦', '˖'];

/// Disegna il campo di particelle su tutta l'area, dietro al resto.
pub fn render(frame: &mut Frame, area: Rect, t: f32, theme: &Theme) {
    let w = area.width as usize;
    let h = area.height as usize;
    if w == 0 || h == 0 {
        return;
    }

    // Colore uniforme e fioco: le particelle non devono distrarre.
    let style = Style::default().fg(theme.particle);
    let mut lines: Vec<Line> = Vec::with_capacity(h);

    for row in 0..h {
        let mut s = String::with_capacity(w);
        for col in 0..w {
            // Ogni colonna ha una fase e una velocita' pseudo-casuali.
            let col_seed = hash(col as u64);
            let speed = 0.25 + (col_seed % 40) as f32 / 100.0;
            let phase = (col_seed % h as u64) as f32;
            let head = (phase + t * speed * 6.0) % h as f32;
            // Distanza verticale dalla "testa" della particella di questa colonna.
            let dist = (row as f32 - head).rem_euclid(h as f32);
            if dist < 1.0 {
                let g = GLYPHS[(hash(col as u64 * 31 + row as u64) as usize) % GLYPHS.len()];
                s.push(g);
            } else {
                s.push(' ');
            }
        }
        lines.push(Line::from(Span::styled(s, style)));
    }

    frame.render_widget(Paragraph::new(lines), area);
}

fn hash(x: u64) -> u64 {
    let mut z = x.wrapping_add(0x9E3779B97F4A7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_is_deterministic() {
        assert_eq!(hash(5), hash(5));
        assert_ne!(hash(5), hash(6));
    }
}
