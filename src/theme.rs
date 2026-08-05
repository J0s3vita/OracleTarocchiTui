//! Palette. "Ritual" (nero gotico + viola/ambra) di default, con la tinta
//! dinamica per carta che ne colora temporaneamente i bordi.

use ratatui::style::{Color, Modifier, Style};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Theme {
    pub name: &'static str,
    pub bg: Color,
    pub fg: Color,
    pub dim: Color,
    pub accent: Color,
    pub accent2: Color,
    pub ok: Color,
    pub warn: Color,
    pub danger: Color,
    pub border: Color,
    /// Colore delle particelle di sfondo.
    pub particle: Color,
}

pub const RITUAL: Theme = Theme {
    name: "Ritual",
    bg: Color::Rgb(8, 6, 12),
    fg: Color::Rgb(210, 198, 230),
    dim: Color::Rgb(96, 84, 120),
    accent: Color::Rgb(170, 130, 240),
    accent2: Color::Rgb(230, 170, 90),
    ok: Color::Rgb(130, 210, 160),
    warn: Color::Rgb(230, 190, 90),
    danger: Color::Rgb(224, 80, 110),
    border: Color::Rgb(72, 56, 104),
    particle: Color::Rgb(70, 56, 110),
};

pub const BLOOD: Theme = Theme {
    name: "Blood Moon",
    bg: Color::Rgb(12, 5, 7),
    fg: Color::Rgb(230, 200, 205),
    dim: Color::Rgb(120, 74, 82),
    accent: Color::Rgb(224, 70, 90),
    accent2: Color::Rgb(230, 150, 90),
    ok: Color::Rgb(180, 200, 120),
    warn: Color::Rgb(232, 170, 90),
    danger: Color::Rgb(255, 70, 70),
    border: Color::Rgb(110, 44, 54),
    particle: Color::Rgb(96, 40, 50),
};

pub const ABYSS: Theme = Theme {
    name: "Abyss",
    bg: Color::Rgb(4, 8, 12),
    fg: Color::Rgb(190, 214, 224),
    dim: Color::Rgb(78, 100, 116),
    accent: Color::Rgb(90, 200, 220),
    accent2: Color::Rgb(120, 160, 240),
    ok: Color::Rgb(120, 220, 190),
    warn: Color::Rgb(220, 200, 120),
    danger: Color::Rgb(230, 100, 120),
    border: Color::Rgb(48, 78, 96),
    particle: Color::Rgb(40, 72, 90),
};

pub const THEMES: [Theme; 3] = [RITUAL, BLOOD, ABYSS];

impl Theme {
    pub fn base(&self) -> Style {
        Style::default().fg(self.fg).bg(self.bg)
    }
    pub fn dimmed(&self) -> Style {
        Style::default().fg(self.dim)
    }
    pub fn title(&self) -> Style {
        Style::default().fg(self.accent).add_modifier(Modifier::BOLD)
    }
    pub fn border_style(&self, focused: bool) -> Style {
        Style::default().fg(if focused { self.accent } else { self.border })
    }
    pub fn selection(&self) -> Style {
        Style::default()
            .fg(self.bg)
            .bg(self.accent)
            .add_modifier(Modifier::BOLD)
    }

    /// Interpola verso lo sfondo: `t=0` sfondo, `t=1` colore pieno.
    pub fn fade(&self, color: Color, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        let (br, bg, bb) = rgb(self.bg).unwrap_or((0, 0, 0));
        match rgb(color) {
            Some((r, g, b)) => Color::Rgb(lerp(br, r, t), lerp(bg, g, t), lerp(bb, b, t)),
            None => color,
        }
    }

    /// Schiarisce verso il bianco, per il glow del reveal.
    pub fn glow(&self, color: Color, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        match rgb(color) {
            Some((r, g, b)) => Color::Rgb(lerp(r, 255, t), lerp(g, 255, t), lerp(b, 255, t)),
            None => color,
        }
    }
}

/// Colore RGB di una carta come `Color`.
pub fn card_color(rgb: [u8; 3]) -> Color {
    Color::Rgb(rgb[0], rgb[1], rgb[2])
}

fn rgb(color: Color) -> Option<(u8, u8, u8)> {
    match color {
        Color::Rgb(r, g, b) => Some((r, g, b)),
        _ => None,
    }
}

fn lerp(from: u8, to: u8, t: f32) -> u8 {
    (from as f32 + (to as f32 - from as f32) * t)
        .round()
        .clamp(0.0, 255.0) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fade_endpoints() {
        let t = RITUAL;
        assert_eq!(t.fade(t.accent, 1.0), t.accent);
        assert_eq!(t.fade(t.accent, 0.0), t.bg);
    }

    #[test]
    fn glow_saturates_white() {
        assert_eq!(RITUAL.glow(RITUAL.accent, 1.0), Color::Rgb(255, 255, 255));
    }

    #[test]
    fn card_color_maps_rgb() {
        assert_eq!(card_color([10, 20, 30]), Color::Rgb(10, 20, 30));
    }
}
