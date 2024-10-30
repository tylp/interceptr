use ratatui::{
    buffer::Buffer,
    layout::Rect,
    symbols::border,
    text::Line,
    widgets::{Block, Widget},
};

pub struct Header {
    title: String,
    menus: Vec<String>,
}

impl Header {
    pub fn new(title: &str, menus: Vec<&str>) -> Self {
        Self {
            title: title.to_string(),
            menus: menus.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl Widget for &Header {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title_top(Line::from(self.title.clone()).centered())
            .border_set(border::THICK);

        block.render(area, buf);
    }
}
