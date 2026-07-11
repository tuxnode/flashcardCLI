use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

pub enum AppEvent {
    Key(KeyEvent),
    Tick,
}

pub struct EventHandler {
    receiver: mpsc::Receiver<AppEvent>,
    _handle: thread::JoinHandle<()>,
}

impl EventHandler {
    pub fn new(tick_rate: Duration) -> Self {
        let (sender, receiver) = mpsc::channel();
        let handle = thread::spawn(move || {
            loop {
                if event::poll(tick_rate).unwrap_or(false) {
                    if let Event::Key(key) = event::read().unwrap_or(Event::Key(KeyEvent::new(
                        KeyCode::Esc,
                        KeyModifiers::NONE,
                    ))) {
                        sender.send(AppEvent::Key(key)).ok();
                    }
                }
                sender.send(AppEvent::Tick).ok();
            }
        });
        Self {
            receiver,
            _handle: handle,
        }
    }

    pub fn next(&self) -> Result<AppEvent, mpsc::RecvError> {
        self.receiver.recv()
    }
}
