use crate::data_buffer::DataBuffer;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Modifier, Style};
use tui::widgets::Widget;

pub struct WaveWidget<'a> {
    waveform: &'a DataBuffer,
}

impl<'a> WaveWidget<'a> {
    pub fn new(waveform: &'a DataBuffer) -> Self {
        Self { waveform }
    }
}

const SAMPLE_RANGE: f32 = std::i16::MAX as f32 - std::i16::MIN as f32 + 1.;

impl<'a> Widget for WaveWidget<'a> {
    fn draw<'b>(&mut self, area: Rect, buf: &'b mut Buffer) {
        let Rect { width, height, .. } = area;
        let waveform_len = self.waveform.len();
        assert!(waveform_len > width.into());
        for (col, &sample) in self
            .waveform
            .iter()
            .skip(waveform_len - usize::from(width))
            .enumerate()
        {
            let row = ((f32::from(sample) - f32::from(std::i16::MIN)) / SAMPLE_RANGE
                * f32::from(height))
            .floor() as u16;
            buf.get_mut(col as u16, row).set_char('X');
        }

        buf.set_string(
            2,
            4,
            "This is from WaveWidget",
            Style::default().modifier(Modifier::BOLD),
        );
    }
}
