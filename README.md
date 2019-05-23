# Rust TUI Audio Example

This is a proof-of-concept learning experiment that uses `tui-rs` and `wavy` (though any audio lib would do) to show a live waveform from the default recording source on the sound card.

Refresh rate is awesome (~50 FPS by choice), live waveform has colors as well.

This was a learning project exploring custom errors, tests, threads, mpsc channels, mutexes, terminal user interfaces, user input, and refresh rate. Now that I've learned enough from this, time to put it on display and move onto the next thing!

This project compiles and runs on Linux and Windows.
