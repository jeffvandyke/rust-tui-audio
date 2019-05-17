use cpal::{EventLoop, StreamData, UnknownTypeInputBuffer};
use std::fmt;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use super::data_buffer::DataBuffer;

pub struct App {
    event_loop: Arc<EventLoop>,
    shared_buffer: Arc<Mutex<DataBuffer>>,
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
            shared_buffer: Arc::new(Mutex::new(DataBuffer::new(500))),
            // stream_id,
        })
    }

    pub fn run(&mut self) -> Result<(), ()> {
        // Start thread for reading audio data
        let shared_buffer = self.shared_buffer.clone();
        let event_loop = self.event_loop.clone();
        std::thread::spawn(move || {
            // const ENVELOPE_SIZE: usize = 32;
            // let leftover...

            event_loop
                .clone()
                .run(|_stream_id, stream_data| match stream_data {
                    StreamData::Input {
                        buffer: UnknownTypeInputBuffer::F32(buffer),
                    } => {
                        let mut unlocked_buffer = shared_buffer.lock().unwrap();
                        unlocked_buffer.push_latest_data(&buffer);
                    }
                    StreamData::Input { .. } => {
                        panic!("Want F32 buffer!!! (suggestion: Jeff, be less picky!");
                    }
                    _ => (),
                });
        });

        let mut x = 0;
        loop {
            std::thread::sleep(Duration::from_millis(10));
            let buffer = self.shared_buffer.lock().unwrap();
            let len = buffer.len();
            let avg = buffer.iter().map(|v| v.abs() / len as f32).sum::<f32>();

            println!("{}", avg);
            for _ in 0..(x % 32) {
                print!(" ");
            }
            println!("O");
            x += 1;
        }
    }
}
