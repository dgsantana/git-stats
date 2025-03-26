use std::{sync::mpsc, thread, time::{Duration, Instant}};

use anyhow::Result;
use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, KeyCode};

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
            let mut last_tick = Instant::now();
            let mut last_key: Option<KeyCode> = None;
            let mut last_key_time = Instant::now();
            // Debounce time in milliseconds
            let debounce_duration = Duration::from_millis(100);
            
            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));

                if event::poll(timeout).unwrap() {
                    if let CrosstermEvent::Key(key) = event::read().unwrap() {
                        // Only send the key event if:
                        // 1. It's a different key than the last one, or
                        // 2. Enough time has passed since the last key press
                        let current_time = Instant::now();
                        if last_key != Some(key.code) || 
                           current_time.duration_since(last_key_time) > debounce_duration {
                            
                            if event_tx.send(Event::Input(key)).is_err() {
                                break;
                            }
                            
                            // Update the last key and time
                            last_key = Some(key.code);
                            last_key_time = current_time;
                        }
                    }
                }

                if last_tick.elapsed() >= tick_rate {
                    if event_tx.send(Event::Tick).is_err() {
                        break;
                    }
                    last_tick = Instant::now();
                }
            }
        });

        Self { rx, _tx: tx }
    }

    pub fn next(&self) -> Result<Event> {
        Ok(self.rx.recv()?)
    }
}
