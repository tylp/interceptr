use ratatui::{
    buffer::Buffer,
    layout::Rect,
    symbols::border,
    text::Line,
    widgets::{Block, Widget},
};

use super::{general, header::Tab};

#[derive(Debug, Default)]
pub struct Body {
    tab: Tab,
}

impl Body {
    pub fn new(tab: Tab) -> Self {
        Self { tab }
    }
}

impl Widget for &Body {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let general = general::General::new();

        match self.tab {
            Tab::General => general.render(area, buf),
            Tab::Sources => todo!("Render general tab"),
            Tab::Targets => todo!("Render general tab"),
            Tab::Rules => todo!("Render general tab"),
        }
    }
}
