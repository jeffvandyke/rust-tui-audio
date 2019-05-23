use crate::data_buffer::DataBuffer;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::Color;
use tui::widgets::Widget;

pub struct WaveWidget<'a> {
    waveform: &'a DataBuffer,
}

impl<'a> WaveWidget<'a> {
    pub fn new(waveform: &'a DataBuffer) -> Self {
        Self { waveform }
    }
}

impl<'a> Widget for WaveWidget<'a> {
    /// Draws the WaveWidget's waveform onto the terminal buffer
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        let Rect { width, height, .. } = area;
        let waveform_len = self.waveform.len();
        assert!(waveform_len > width.into());

        for col in 1..=width {
            buf.get_mut(col, height / 2)
                .set_char('=')
                .set_fg(Color::Green);
        }

        for (index, &sample) in self
            .waveform
            .iter()
            .skip(waveform_len - usize::from(width))
            .enumerate()
        {
            let col = index as u16 + 1;
            // Scale (might clip) sample to see more
            let norm_y = sample * 5.;

            let row = ((norm_y + 0.5) * f32::from(height)).floor() as u16;

            // If would clip, don't render anything
            if row > 0 && row < height {
                buf.get_mut(col, row).set_char('#').set_fg(Color::Cyan);
            }
        }
    }
}
