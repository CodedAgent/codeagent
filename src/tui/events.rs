use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

pub enum AppEvent {
    Tick,
    Input(KeyEvent),
}

pub struct EventHandler {
    tx: Sender<AppEvent>,
    rx: Receiver<AppEvent>,
}

impl EventHandler {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        let tx_clone = tx.clone();

        thread::spawn(move || {
            let tick_rate = Duration::from_millis(200);
            loop {
                if event::poll(tick_rate).ok().unwrap_or(false) {
                    if let Ok(Event::Key(key)) = event::read() {
                        let _ = tx_clone.send(AppEvent::Input(key));
                    }
                }
                let _ = tx_clone.send(AppEvent::Tick);
            }
        });

        EventHandler { tx, rx }
    }

    pub fn next(&self) -> Option<AppEvent> {
        self.rx.try_recv().ok()
    }
}
