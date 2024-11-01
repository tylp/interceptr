use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    symbols::{self, border},
    text::Line,
    widgets::{Block, List, ListDirection, ListState, StatefulWidget, Tabs, Widget},
};

#[derive(Default, Clone, Copy, Debug, Eq, PartialEq)]
pub struct General;

impl General {
    pub fn new() -> Self {
        Self
    }
}

impl Widget for &General {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(50), // Space for header (adjust as needed)
                    Constraint::Min(0),         // Remaining space for main content
                ]
                .as_ref(),
            )
            .split(area);

        let sources_list = SourcesList::default();
        let right_pane = Transformer::default();

        sources_list.render(chunks[0], buf);
        right_pane.render(chunks[1], buf);
    }
}

#[derive(Debug, Default)]
pub struct SourcesList {
    state: ListState,
}

impl Widget for SourcesList {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let items = vec!["Item1", "Item2", "Item3"];
        let mut state = ListState::default();

        state.select(Some(1));

        let list = List::new(items)
            .block(Block::bordered().title("Sources"))
            .style(Style::new().white())
            .highlight_style(Style::new().bold().italic().light_cyan())
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom);

        ratatui::prelude::StatefulWidget::render(list, area, buf, &mut state);
    }
}

#[derive(Default, Debug)]
pub struct Transformer {}

impl Widget for Transformer {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(50), // Space for header (adjust as needed)
                    Constraint::Min(0),         // Remaining space for main content
                ]
                .as_ref(),
            )
            .split(area);

        let source_buffer = Block::bordered()
            .border_set(border::THICK)
            .title_top("Source");

        let target_buffer = Block::bordered()
            .border_set(border::THICK)
            .title_top("Target");

        source_buffer.render(chunks[0], buf);
        target_buffer.render(chunks[1], buf);
    }
}
