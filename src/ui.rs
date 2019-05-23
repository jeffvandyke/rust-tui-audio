use crate::app::App;
use crossterm;
use std::io;
use std::sync::mpsc;
use tui::backend::CrosstermBackend;
use tui::Terminal;

pub use crossterm::KeyEvent;

mod wave_widget;
use wave_widget::WaveWidget;

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
        terminal.clear()?;
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
                                panic!("Problem with tx.send");
                            }
                        }
                        None => {
                            std::thread::sleep(std::time::Duration::from_millis(100));
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
        use tui::layout;
        use tui::widgets::{self, Block, Borders, Text, Widget};

        let buffer = app.shared_buffer.lock().unwrap();
        let len = buffer.len();
        let avg = buffer.iter().map(|v| v.abs() / len as f32).sum::<f32>();

        let text = [
            Text::raw(format!("X is: {}", app.x)),
            Text::raw(" - "),
            Text::raw(format!("Average is: {}", avg)),
        ];

        self.terminal.draw(|mut frame| {
            let chunks = layout::Layout::default()
                .direction(layout::Direction::Vertical)
                .constraints([layout::Constraint::Min(3), layout::Constraint::Length(3)].as_ref())
                .split(frame.size());

            let mut waveform_block = Block::default()
                .borders(Borders::ALL)
                .title("Waveform Oscilloscope");
            waveform_block.render(&mut frame, chunks[0]);

            WaveWidget::new(&buffer).render(&mut frame, waveform_block.inner(chunks[0]));

            widgets::Paragraph::new(text.iter())
                .block(Block::default().borders(Borders::ALL).title("Status"))
                .render(&mut frame, chunks[1]);
        })?;

        Ok(())
    }
}
