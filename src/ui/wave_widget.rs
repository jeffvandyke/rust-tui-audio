use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Modifier, Style};
use tui::widgets::Widget;

pub struct WaveWidget {
    // buffer: crate::data_buffer::DataBuffer,
}

impl WaveWidget {
    pub fn new() -> Self {
        Self {}
    }
}

impl Widget for WaveWidget {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        buf.set_string(
            2,
            4,
            "This is from WaveWidget",
            Style::default().modifier(Modifier::BOLD),
        );
    }
}
