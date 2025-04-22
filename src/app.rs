use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, KeyEventKind};
use ratatui::{prelude::*, DefaultTerminal};

pub struct App {
    mode: Mode,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum Mode {
    #[default]
    Running,
    Quit,
}

impl App {
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.is_running() {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.mode != Mode::Quit
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> Result<()> {
        let timeout = Duration::from_secs_f64(1.0 / 50.0);
        if !event::poll(timeout)? {
            return Ok(());
        }

        match event::read()? {
            event::Event::Key(key) if key.kind == KeyEventKind::Press => {
                self.handle_key_press(key.code);
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_press(&mut self, key: event::KeyCode) {
        match key {
            event::KeyCode::Esc => self.mode = Mode::Quit,
            _ => {}
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        Span::styled("Git Stats", Style::default().fg(Color::White))
            .render(area, buf);
    }
}
