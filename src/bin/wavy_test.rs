use wavy::*;

use std::collections::VecDeque;

fn main() -> Result<(), AudioError> {
    // Connect to the speaker and microphone systems.
    let mut mic = MicrophoneSystem::new(SampleRate::Normal)?;
    let mut speaker = SpeakerSystem::new(SampleRate::Normal)?;

    let mut buffer = VecDeque::new();

    loop {
        // Record some sound.
        mic.record(&mut |_whichmic, l, r| {
            buffer.push_back((l, r));
        });

        // Play that sound.
        speaker.play(&mut || {
            if let Some((lsample, rsample)) = buffer.pop_front() {
                AudioSample::stereo(lsample, rsample)
            } else {
                // Play silence if not enough has been recorded yet.
                AudioSample::stereo(0, 0)
            }
        });
    }
}
