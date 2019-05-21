use crate::data_buffer::DataBuffer;
use crate::ui;
use cpal::{EventLoop, StreamData, UnknownTypeInputBuffer};
use std::fmt;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct App {
    event_loop: Arc<EventLoop>,
    pub shared_buffer: Arc<Mutex<DataBuffer>>,
    pub x: i64,
    // stream_id: cpal::StreamId,
}

// begin error boilerplate (might replace with a crate-provided automation

#[derive(Debug)]
pub enum InitError {
    NoDefaultAudioInput,
    DefaulAudioFormatError(cpal::DefaultFormatError),
    StreamCreationError(cpal::CreationError),
    InitUiError(std::io::Error),
}

impl fmt::Display for InitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                InitError::NoDefaultAudioInput => {
                    "Couldn't initialize default input device".to_string()
                }
                InitError::DefaulAudioFormatError(err) => format!(
                    "Error obtaining default format for a valid input device: {}",
                    err
                ),
                InitError::StreamCreationError(err) => format!("Error creating stream: {}", err),
                InitError::InitUiError(io_err) => format!("Error creating stream: {}", io_err),
            }
        )
    }
}

impl From<cpal::DefaultFormatError> for InitError {
    fn from(err: cpal::DefaultFormatError) -> Self {
        InitError::DefaulAudioFormatError(err)
    }
}

impl From<cpal::CreationError> for InitError {
    fn from(err: cpal::CreationError) -> Self {
        InitError::StreamCreationError(err)
    }
}

impl From<std::io::Error> for InitError {
    fn from(err: std::io::Error) -> Self {
        InitError::InitUiError(err)
    }
}

// }}} end error boilerplate

impl App {
    /// Initializes (configures) soundcard devices, prime for running.
    pub fn init() -> Result<Self, InitError> {
        // setup incoming stream as per cpal module docs (except with build_input_stream)
        let event_loop = EventLoop::new();
        let input_device = cpal::default_input_device().ok_or(InitError::NoDefaultAudioInput)?;
        let default_format = input_device.default_input_format()?;
        let _stream_id = event_loop.build_input_stream(&input_device, &default_format)?;

        Ok(Self {
            event_loop: Arc::new(event_loop),
            shared_buffer: Arc::new(Mutex::new(DataBuffer::new(500))),
            x: 0,
        })
    }

    pub fn on_key(&mut self, key: char) {
        if key == 'r' {
            // Reset 'x'
            self.x = 0;
        }
    }

    pub fn run(&mut self, ui: &mut ui::Ui) -> Result<(), ()> {
        // Start thread for reading audio data
        let shared_buffer = self.shared_buffer.clone();
        let event_loop = self.event_loop.clone();
        std::thread::spawn(move || {
            // const ENVELOPE_SIZE: usize = 32;
            // let leftover...
            let mut quick_tmp_buffer = vec![0.0; 1000];
            event_loop.run(|_stream_id, stream_data| match stream_data {
                StreamData::Input {
                    buffer: UnknownTypeInputBuffer::F32(buffer),
                } => {
                    let buffer_len = buffer.len();
                    if buffer_len > quick_tmp_buffer.len() {
                        quick_tmp_buffer.resize(buffer_len, 0.);
                    }
                    // Now quick_tmp_buffer is large enough to hold elements, use it as a tmp
                    // storage to get data out of the buffer as quickly as possible!!!
                    // (still crashes sometimes, TODO: fix)

                    quick_tmp_buffer[..buffer_len].copy_from_slice(&buffer);

                    let mut unlocked_buffer = shared_buffer.lock().unwrap();
                    unlocked_buffer.push_latest_data(&quick_tmp_buffer[..buffer_len]);
                }
                StreamData::Input { .. } => {
                    panic!("Want F32 buffer!!! (suggestion: Jeff, be less picky!");
                }
                _ => (),
            });
        });

        loop {
            // Process all available inputs key_event_rx
            loop {
                use std::sync::mpsc::TryRecvError; // shortcut
                match ui.key_event_rx.try_recv() {
                    Ok(event) => match event {
                        // Only 1 kind of event for now...
                        ui::Event::Input(key) => self.on_key(key),
                    },
                    // Done looping
                    Err(TryRecvError::Empty) => break,
                    // Done RUNNING, exit...
                    Err(TryRecvError::Disconnected) => return Ok(()),
                }
            }

            self.x += 1;
            if self.x > 1000 {
                return Ok(());
            }

            ui.draw(&self)
                .expect("Failure calling ui.draw, aborting...");

            // Wait until next loop (~16.66 ms)
            std::thread::sleep(Duration::from_millis(1000 / 60));
        }
    }
}
