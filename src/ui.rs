use crate::app::App;
use crossterm;
use std::io;
use std::sync::mpsc;
use tui::backend::CrosstermBackend;
use tui::Terminal;

pub enum Event {
    Input(char),
}

pub struct Ui {
    terminal: Terminal<CrosstermBackend>,
    pub key_event_rx: mpsc::Receiver<Event>,
}

impl Ui {
    pub fn init() -> Result<Self, io::Error> {
        let backend = CrosstermBackend::new();
        let mut terminal = Terminal::new(backend)?;
        terminal.hide_cursor()?;

        let (tx, rx) = mpsc::channel();

        {
            let tx = tx.clone();
            std::thread::spawn(move || {
                let input = crossterm::input();
                loop {
                    if let Ok(key) = input.read_char() {
                        if tx.send(Event::Input(key)).is_err() {
                            return;
                        }
                        if key == '' {
                            // Ctrl-c
                            return;
                        }
                    }
                    // TODO: is this a performance drain?
                }
            });
        }

        Ok(Self {
            terminal,
            key_event_rx: rx,
        })
    }

    /// Draw screen for the current app state
    pub fn draw(&mut self, app: &App) -> io::Result<()> {
        use tui::widgets::{self, Text, Widget};
        let size = self.terminal.size()?;

        let buffer = app.shared_buffer.lock().unwrap();
        let len = buffer.len();
        let avg = buffer.iter().map(|v| v.abs() / len as f32).sum::<f32>();

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
