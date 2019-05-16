use std::fmt;
use cpal::{EventLoop};

pub struct App {
    event_loop: EventLoop,
    input: cpal::Device,
}

#[derive(Debug)]
pub enum InitError {
    NoDefaultInputDevice,
}

impl fmt::Display for InitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            InitError::NoDefaultInputDevice => "Couldn't initialize default input device"
        })
    }
}

impl App {
    /// Initializes (configures) soundcard devices, prime for running.
    pub fn init() -> Result<Self, InitError> {
        let input = cpal::default_input_device().ok_or(InitError::NoDefaultInputDevice)?;
        dbg!(input.default_input_format());
        Ok(Self {
            event_loop: EventLoop::new(),
            input,
        })
    }

    pub fn run(&mut self) -> Result<(), ()> {
        loop {

        }
    }
}
