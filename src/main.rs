use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Widget,
    DefaultTerminal, Frame,
};
use ui::header::Tab;

mod iptables;
mod nfqueue;
mod rules;
mod ui;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug, Default)]
pub struct App {
    current_tab: Tab,
    exit: bool,
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    /// updates the application's state based on user input
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.left(),
            KeyCode::Right => self.right(),
            KeyCode::Up => self.up(),
            KeyCode::Down => self.down(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn left(&mut self) {
        match self.current_tab {
            Tab::General => self.current_tab = Tab::Rules,
            Tab::Sources => self.current_tab = Tab::General,
            Tab::Targets => self.current_tab = Tab::Sources,
            Tab::Rules => self.current_tab = Tab::Targets,
        }
    }

    fn right(&mut self) {
        match self.current_tab {
            Tab::General => self.current_tab = Tab::Sources,
            Tab::Sources => self.current_tab = Tab::Targets,
            Tab::Targets => self.current_tab = Tab::Rules,
            Tab::Rules => self.current_tab = Tab::General,
        }
    }

    fn up(&mut self) {}

    fn down(&mut self) {}
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Split the area into a top section for the header and the rest for the main content
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(3), // Space for header (adjust as needed)
                    Constraint::Min(0),    // Remaining space for main content
                ]
                .as_ref(),
            )
            .split(area);

        let header = ui::header::Header::new(self.current_tab);
        let body = ui::body::Body::new(self.current_tab);
        header.render(chunks[0], buf); // Rendering header in the top chunk
        body.render(chunks[1], buf); // Rendering the counter in the remaining space
    }
}
