use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    symbols::{self, border},
    text::Line,
    widgets::{Block, Tabs, Widget},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Tab {
    General,
    Sources,
    Targets,
    Rules,
}

impl Default for Tab {
    fn default() -> Self {
        Self::General
    }
}

impl Tab {
    /// Returns the available tabs.
    const fn get_headers() -> &'static [&'static str] {
        &["General", "Sources", "Targets", "Rules"]
    }
}

impl From<usize> for Tab {
    fn from(v: usize) -> Self {
        match v {
            0 => Self::General,
            1 => Self::Sources,
            2 => Self::Targets,
            3 => Self::Rules,
            _ => Self::default(),
        }
    }
}

pub struct Header {
    title: String,
    menus: Vec<String>,
    tab: Tab,
}

impl Header {
    pub fn new(tab: Tab) -> Self {
        Self {
            title: " Interceptr ".to_string(),
            menus: Tab::get_headers().iter().map(|s| s.to_string()).collect(),
            tab,
        }
    }
}

impl Widget for &Header {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let instructions = " Use ◄ ► to change tab, ▲ ▼  to scroll, q to quit ";

        let block = Block::bordered()
            .title_top(Line::from(self.title.clone()).centered())
            .title_bottom(Line::from(instructions).centered())
            .border_set(border::THICK);

        let tab = Tabs::new(self.menus.clone())
            .highlight_style(Style::default().yellow())
            .divider(symbols::line::VERTICAL)
            .select(self.tab as usize)
            .block(block);

        tab.render(area, buf);
    }
}
