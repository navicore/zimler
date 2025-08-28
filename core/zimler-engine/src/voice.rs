use crate::{Envelope, Sample};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VoiceState {
    Idle,
    Active,
    Releasing,
}

pub struct Voice {
    state: VoiceState,
    sample: Option<Sample>,
    position: f64,
    pitch_ratio: f64,
    envelope: Envelope,
    #[allow(dead_code)]
    sample_rate: f32,
    note: Option<u8>,
    velocity: f32,
}

impl Voice {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            state: VoiceState::Idle,
            sample: None,
            position: 0.0,
            pitch_ratio: 1.0,
            envelope: Envelope::new(sample_rate),
            sample_rate,
            note: None,
            velocity: 1.0,
        }
    }

    pub fn trigger(&mut self, note: u8, velocity: f32, sample: Sample) {
        self.note = Some(note);
        self.velocity = velocity;
        self.sample = Some(sample);
        self.position = 0.0;
        self.state = VoiceState::Active;

        // Calculate pitch ratio for 1V/oct (each semitone up = ratio * 2^(1/12))
        let root_note = self.sample.as_ref().and_then(|s| s.root_note).unwrap_or(60);
        let semitones = note as f64 - root_note as f64;
        self.pitch_ratio = 2.0_f64.powf(semitones / 12.0);

        self.envelope.trigger();
    }

    pub fn release(&mut self) {
        if self.state == VoiceState::Active {
            self.state = VoiceState::Releasing;
            self.envelope.release();
        }
    }

    pub fn is_active(&self) -> bool {
        self.state != VoiceState::Idle
    }

    pub fn get_note(&self) -> Option<u8> {
        self.note
    }

    pub fn process_block(&mut self, output: &mut [f32]) {
        if let Some(sample) = &self.sample {
            let channels = sample.channels;
            let sample_data = &sample.data;
            let sample_len = sample_data.len() / channels;

            for out in output.chunks_mut(channels) {
                if self.position >= sample_len as f64 {
                    self.state = VoiceState::Idle;
                    break;
                }

                // Linear interpolation for sub-sample accuracy
                let pos_floor = self.position.floor() as usize;
                let pos_fract = self.position.fract() as f32;

                for ch in 0..channels.min(out.len()) {
                    let idx = pos_floor * channels + ch;
                    let next_idx = ((pos_floor + 1) * channels + ch).min(sample_data.len() - 1);

                    let sample_value = if idx < sample_data.len() {
                        let curr = sample_data[idx];
                        let next = sample_data[next_idx];
                        curr * (1.0 - pos_fract) + next * pos_fract
                    } else {
                        0.0
                    };

                    out[ch] += sample_value * self.velocity * self.envelope.get_current_value();
                }

                self.position += self.pitch_ratio;
                self.envelope.process_sample();

                if self.envelope.is_finished() {
                    self.state = VoiceState::Idle;
                }
            }
        }
    }
}
