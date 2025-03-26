use std::{sync::mpsc, thread, time::Duration};

use anyhow::Result;
use crossterm::event::{self, Event as CrosstermEvent, KeyEvent};

pub enum Event {
    Input(KeyEvent),
    Tick,
}

pub struct EventHandler {
    rx: mpsc::Receiver<Event>,
    _tx: mpsc::Sender<Event>, // Keep sender alive
}

impl EventHandler {
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = mpsc::channel();
        let event_tx = tx.clone();

        thread::spawn(move || {
            let mut last_tick = std::time::Instant::now();
            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));

                if event::poll(timeout).unwrap() {
                    if let CrosstermEvent::Key(key) = event::read().unwrap() {
                        if event_tx.send(Event::Input(key)).is_err() {
                            break;
                        }
                    }
                }

                if last_tick.elapsed() >= tick_rate {
                    if event_tx.send(Event::Tick).is_err() {
                        break;
                    }
                    last_tick = std::time::Instant::now();
                }
            }
        });

        Self { rx, _tx: tx }
    }

    pub fn next(&self) -> Result<Event> {
        Ok(self.rx.recv()?)
    }
}
