use ratatui::style::{Color, Modifier, Style};

#[derive(Debug, Clone)]
pub struct Theme {
    pub bg: Color,
    pub panel: Color,
    pub panel_alt: Color,
    pub fg: Color,
    pub dim: Color,
    pub accent: Color,
    pub ok: Color,
    pub warn: Color,
    pub danger: Color,
    pub border: Color,
    pub border_focus: Color,
    pub select_bg: Color,
    pub select_fg: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            bg: Color::Reset,
            panel: Color::Rgb(18, 22, 30),
            panel_alt: Color::Rgb(14, 18, 26),
            fg: Color::Rgb(224, 231, 240),
            dim: Color::Rgb(129, 145, 166),
            accent: Color::Rgb(77, 166, 255),
            ok: Color::Rgb(59, 214, 123),
            warn: Color::Rgb(255, 190, 92),
            danger: Color::Rgb(255, 95, 109),
            border: Color::Rgb(60, 70, 86),
            border_focus: Color::Rgb(77, 166, 255),
            select_bg: Color::Rgb(28, 40, 60),
            select_fg: Color::Rgb(224, 231, 240),
        }
    }
}

impl Theme {
    pub fn root(&self) -> Style {
        Style::default().bg(self.bg).fg(self.fg)
    }

    pub fn panel(&self) -> Style {
        Style::default().bg(self.panel).fg(self.fg)
    }

    pub fn panel_alt(&self) -> Style {
        Style::default().bg(self.panel_alt).fg(self.fg)
    }

    pub fn title(&self) -> Style {
        Style::default().fg(self.accent).add_modifier(Modifier::BOLD)
    }

    pub fn border(&self) -> Style {
        Style::default().fg(self.border)
    }

    pub fn border_focused(&self) -> Style {
        Style::default().fg(self.border_focus)
    }

    pub fn dim(&self) -> Style {
        Style::default().fg(self.dim)
    }

    pub fn selected(&self) -> Style {
        Style::default()
            .bg(self.select_bg)
            .fg(self.select_fg)
            .add_modifier(Modifier::BOLD)
    }

    pub fn badge_ok(&self) -> Style {
        Style::default().fg(self.ok).add_modifier(Modifier::BOLD)
    }

    pub fn badge_warn(&self) -> Style {
        Style::default().fg(self.warn).add_modifier(Modifier::BOLD)
    }

    pub fn badge_danger(&self) -> Style {
        Style::default().fg(self.danger).add_modifier(Modifier::BOLD)
    }
}
