use crate::app::App;
use crossterm;
use std::io;
use std::sync::mpsc;
use tui::backend::CrosstermBackend;
use tui::Terminal;

pub use crossterm::KeyEvent;

pub enum Event {
    KeyInput(KeyEvent),
}

pub struct Ui {
    terminal: Terminal<CrosstermBackend>,
    _into_raw_drop_reset: crossterm::RawScreen,
    pub key_event_rx: mpsc::Receiver<Event>,
}

impl Ui {
    pub fn init() -> Result<Self, io::Error> {
        let backend = CrosstermBackend::new();
        let mut terminal = Terminal::new(backend)?;
        terminal.hide_cursor()?;

        let into_raw_drop_reset =
            crossterm::RawScreen::into_raw_mode().expect("Couldn't put in raw mode");

        let (tx, rx) = mpsc::channel();

        {
            let tx = tx.clone();
            std::thread::spawn(move || {
                let input = crossterm::input();
                let mut stdin = input.read_sync();
                loop {
                    use crossterm::InputEvent;
                    match stdin.next() {
                        Some(InputEvent::Keyboard(key_event)) => {
                            if let KeyEvent::Ctrl('c') = key_event {
                                return;
                            }
                            if tx.send(Event::KeyInput(key_event)).is_err() {
                                return;
                            }
                        }
                        None => {
                            return;
                        }
                        _ => {}
                    }
                    // TODO: is this a performance drain?
                }
            });
        }

        Ok(Self {
            terminal,
            _into_raw_drop_reset: into_raw_drop_reset,
            key_event_rx: rx,
        })
    }

    /// Draw screen for the current app state
    pub fn draw(&mut self, app: &App) -> io::Result<()> {
        use tui::widgets::{self, Text, Widget};
        let size = self.terminal.size()?;

        let buffer = app.shared_buffer.lock().unwrap();
        let len = buffer.len();
        let avg = buffer.iter().map(|v| f32::from(v.abs()) / len as f32).sum::<f32>();

        let text = [
            Text::raw(format!("X is: {}", app.x)),
            Text::raw(format!("Average is: {}", avg)),
        ];

        self.terminal.draw(|mut frame| {
            widgets::Paragraph::new(text.iter()).render(&mut frame, size);
        })?;

        Ok(())
    }
}
