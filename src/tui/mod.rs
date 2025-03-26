pub mod app;
pub mod event;
pub mod ui;

use std::io;
use std::time::Duration;

use anyhow::Result;
use app::App;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, prelude::CrosstermBackend};

use self::event::{Event, EventHandler};
use crate::users::UserInfo;

pub fn run_tui(users: Vec<UserInfo>) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it
    let tick_rate = Duration::from_millis(250);
    let app = App::new(users);
    let res = run_app(&mut terminal, app, tick_rate);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> Result<()> {
    let events = EventHandler::new(tick_rate);

    loop {
        terminal.draw(|f| ui::render(f, &mut app))?;

        match events.next()? {
            Event::Input(key) => match key.code {
                crossterm::event::KeyCode::Char('q') => return Ok(()),
                crossterm::event::KeyCode::Down => app.next(),
                crossterm::event::KeyCode::Up => app.previous(),
                crossterm::event::KeyCode::Tab => app.toggle_focus(),
                _ => {}
            },
            Event::Tick => {
                app.on_tick();
            }
        }

        if app.should_quit {
            return Ok(());
        }
    }
}
