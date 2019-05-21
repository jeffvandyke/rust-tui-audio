use crate::data_buffer::DataBuffer;
use std::io;
use crossterm;
use tui::backend::CrosstermBackend;
use tui::Terminal;

struct UiContext {
    x: i32,
}

impl UiContext {
    fn new() -> Self {
        Self { x: 0, }
    }
}

pub struct Ui {
    terminal: Terminal<CrosstermBackend>,
    context: UiContext,
}

impl Ui {
    pub fn init() -> Result<Self, io::Error> {
        let backend = CrosstermBackend::new();
        let mut terminal = Terminal::new(backend)?;
        terminal.hide_cursor()?;
        Ok(Self {
            context: UiContext::new(),
            terminal,
        })
    }

    /// Draw 1 rendering
    pub fn draw(&mut self, audio_buffer: &DataBuffer) {
        let len = audio_buffer.len();
        let avg = audio_buffer.iter().map(|v| v.abs() / len as f32).sum::<f32>();
        let ctx = &mut self.context;
        let x = &mut ctx.x;

        *x += 1;
    }
}
