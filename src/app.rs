use cpal::{EventLoop, StreamData, UnknownTypeInputBuffer};
use std::fmt;
use std::sync::Arc;
use std::time::Duration;

pub struct App {
    event_loop: Arc<EventLoop>,
    // stream_id: cpal::StreamId,
}

// begin error boilerplate (might replace with a crate-provided automation

#[derive(Debug)]
pub enum InitError {
    NoDefaultInputDevice,
    DefaultFormatError(cpal::DefaultFormatError),
    CreationError(cpal::CreationError),
}

impl fmt::Display for InitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                InitError::NoDefaultInputDevice => "Couldn't initialize default input device",
                InitError::DefaultFormatError(_) => {
                    "Error obtaining default format for a valid input device"
                }
                InitError::CreationError(_) => "Error creating stream",
            }
        )
    }
}

impl From<cpal::DefaultFormatError> for InitError {
    fn from(err: cpal::DefaultFormatError) -> Self {
        InitError::DefaultFormatError(err)
    }
}

impl From<cpal::CreationError> for InitError {
    fn from(err: cpal::CreationError) -> Self {
        InitError::CreationError(err)
    }
}

// }}} end error boilerplate

impl App {
    /// Initializes (configures) soundcard devices, prime for running.
    pub fn init() -> Result<Self, InitError> {
        // setup incoming stream as per cpal module docs (except with build_input_stream)
        let event_loop = EventLoop::new();
        let input_device = cpal::default_input_device().ok_or(InitError::NoDefaultInputDevice)?;
        let default_format = input_device.default_input_format()?;
        let _stream_id = event_loop.build_input_stream(&input_device, &default_format)?;

        Ok(Self {
            event_loop: Arc::new(event_loop),
            // stream_id,
        })
    }

    pub fn run(&mut self) -> Result<(), ()> {
        // Start thread for reading audio data
        let event_loop = self.event_loop.clone();
        std::thread::spawn(move || {
            event_loop
                .clone()
                .run(|_stream_id, stream_data| match stream_data {
                    StreamData::Input {
                        buffer: UnknownTypeInputBuffer::F32(buffer),
                    } => {
                        println!("[{}] {}", buffer.len(), buffer[0].abs());
                    }
                    StreamData::Input { .. } => {
                        panic!("Want F32 buffer!!! (suggestion: Jeff, be less picky!");
                    }
                    _ => (),
                });
        });

        loop {
            println!("...tick...");
            std::thread::sleep(Duration::from_secs(1));
        }
    }
}
