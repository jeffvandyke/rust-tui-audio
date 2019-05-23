use crate::data_buffer::DataBuffer;
use crate::ui;
use std::fmt;
use std::sync::{Arc, Mutex};
use std::time::Duration;

const MAX_BUFFER_SAMPLES: usize = 1000;

pub struct App {
    pub shared_buffer: Arc<Mutex<DataBuffer>>,
    pub x: i64,
}

// begin error boilerplate (might replace with a crate-provided automation

#[derive(Debug)]
pub enum InitError {
    // AudioError(wavy::AudioError),
    InitUiError(std::io::Error),
}

impl fmt::Display for InitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                // InitError::AudioError(err) => format!("Error with audio: {:?}", err),
                InitError::InitUiError(io_err) => format!("Error creating stream: {}", io_err),
            }
        )
    }
}

// impl From<wavy::AudioError> for InitError {
//     fn from(err: wavy::AudioError) -> Self {
//         InitError::AudioError(err)
//     }
// }

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
        Ok(Self {
            shared_buffer: Arc::new(Mutex::new(DataBuffer::new(MAX_BUFFER_SAMPLES))),
            x: 0,
        })
    }

    pub fn on_key(&mut self, key: ui::KeyEvent) {
        use ui::KeyEvent;
        if let KeyEvent::Char('r') = key {
            // Reset 'x'
            self.x = 0;
        }
    }

    pub fn run(&mut self, ui: &mut ui::Ui) -> Result<(), ()> {
        // Start thread for reading audio data
        let shared_buffer = self.shared_buffer.clone();

        let _audio_thread_handle = std::thread::spawn(move || {
            // TODO: Refactor
            #[cfg(target_os = "linux")]
            {
                use wavy;
                let mut mic = wavy::MicrophoneSystem::new(wavy::SampleRate::Normal)
                    .expect("Failed initing mic system");
                const BUF_MAX: usize = 44100 / 500;
                let mut buffer = Vec::with_capacity(BUF_MAX * 2);
                loop {
                    mic.record(&mut |_whichmic, l, _r| {
                        buffer.push(f32::from(l) / f32::from(std::i16::MAX));
                    });

                    if buffer.len() >= BUF_MAX {
                        let mut unlocked_buffer = shared_buffer.lock().unwrap();
                        unlocked_buffer.push_latest_data(&buffer);
                        buffer.clear();
                    }
                }
            }

            // TODO: Refactor
            #[cfg(target_os = "windows")]
            {
                use cpal;
                // setup incoming stream as per cpal module docs (except with build_input_stream)
                let event_loop = cpal::EventLoop::new();
                let input_device =
                    cpal::default_input_device().expect("Failed cpal default_input_device");
                let default_format = input_device
                    .default_input_format()
                    .expect("Failed cpal device default_input_format");
                let _stream_id = event_loop
                    .build_input_stream(&input_device, &default_format)
                    .expect("Failed build_input_stream");

                let mut tmp_buffer = Vec::with_capacity(2000);

                event_loop.run(move |_stream_id, stream_data| {
                    // Normalize the different types
                    match stream_data {
                        cpal::StreamData::Input {
                            buffer: cpal::UnknownTypeInputBuffer::F32(cpal_buffer),
                        } => {
                            cpal_buffer.iter().for_each(|&x_f32| tmp_buffer.push(x_f32));
                        }
                        cpal::StreamData::Input {
                            buffer: cpal::UnknownTypeInputBuffer::U16(cpal_buffer),
                        } => {
                            cpal_buffer.iter().for_each(|&x_u16| {
                                tmp_buffer.push(f32::from(x_u16) / f32::from(std::i16::MAX) - 1.)
                            });
                        }
                        cpal::StreamData::Input {
                            buffer: cpal::UnknownTypeInputBuffer::I16(cpal_buffer),
                        } => {
                            cpal_buffer.iter().for_each(|&x_i16| {
                                tmp_buffer.push(f32::from(x_i16) / f32::from(std::i16::MAX))
                            });
                        }
                        _ => (),
                    };

                    let mut unlocked_buffer = shared_buffer.lock().unwrap();
                    unlocked_buffer.push_latest_data(tmp_buffer.as_slice());
                    tmp_buffer.clear();
                });
            }
        });

        loop {
            // Process all available inputs key_event_rx
            loop {
                use std::sync::mpsc::TryRecvError; // shortcut
                match ui.key_event_rx.try_recv() {
                    Ok(event) => match event {
                        // Only 1 kind of event for now...
                        ui::Event::KeyInput(key) => self.on_key(key),
                    },
                    // Done looping
                    Err(TryRecvError::Empty) => break,
                    // Done RUNNING, exit...
                    Err(TryRecvError::Disconnected) => return Ok(()),
                }
            }

            self.x += 1;

            ui.draw(&self)
                .expect("Failure calling ui.draw, aborting...");

            // Wait until next loop (~16.66 ms)
            std::thread::sleep(Duration::from_millis(1000 / 60));
        }
    }
}
