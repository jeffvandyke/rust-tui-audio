use crate::data_buffer::DataBuffer;
use std::io;
use termion::raw::{IntoRawMode, RawTerminal};
use tui::backend::TermionBackend;
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
    terminal: Terminal<TermionBackend<RawTerminal<io::Stdout>>>,
    context: UiContext,
}

impl Ui {
    pub fn init() -> Result<Self, io::Error> {
        let stdout = io::stdout().into_raw_mode()?;
        let backend = TermionBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
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
