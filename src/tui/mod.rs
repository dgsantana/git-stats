pub mod app;
pub mod ui;

use std::io;
use std::time::Duration;

use anyhow::Result;
use app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, prelude::CrosstermBackend};

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
    loop {
        terminal.draw(|f| ui::render(f, &mut app))?;
        event::poll(tick_rate)?;
        handle_events(&mut app)?;
        if app.should_quit {
            return Ok(());
        }
    }
}

fn handle_events(app: &mut App) -> Result<()> {
    let timeout = Duration::from_secs_f64(1.0 / 50.0);
    if !event::poll(timeout)? {
        return Ok(());
    }

    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char('q') => return Ok(()),
            KeyCode::Down => app.next(),
            KeyCode::Up => app.previous(),
            KeyCode::Tab => app.toggle_focus(),
            KeyCode::Esc => app.should_quit = true,
            _ => (),
        },
        _ => (),
    }
    Ok(())
}
