use crate::api::MixMode;

pub struct Mixer {
    mode: MixMode,
    master_volume: f32,
    blur_buffer: Vec<f32>,
    blur_position: usize,
}

impl Default for Mixer {
    fn default() -> Self {
        Self::new()
    }
}

impl Mixer {
    pub fn new() -> Self {
        Self {
            mode: MixMode::Poly,
            master_volume: 0.8,
            blur_buffer: vec![0.0; 4096],
            blur_position: 0,
        }
    }

    pub fn set_mode(&mut self, mode: MixMode) {
        self.mode = mode;
        if let MixMode::Blur { crossfade_ms } = mode {
            let buffer_size = (crossfade_ms * 48.0) as usize; // Assuming 48kHz
            self.blur_buffer.resize(buffer_size, 0.0);
        }
    }

    pub fn mix(&mut self, input: &[f32], output: &mut [f32]) {
        match self.mode {
            MixMode::Poly => {
                // Standard mixing - just add
                for (i, sample) in input.iter().enumerate() {
                    output[i] += sample * self.master_volume;
                }
            }
            MixMode::Blur { crossfade_ms } => {
                // Serge-style blurring with overlap
                let blur_len = (crossfade_ms * 48.0) as usize;

                for (i, sample) in input.iter().enumerate() {
                    // Mix with previous content in circular buffer
                    let blur_idx = (self.blur_position + i) % blur_len;
                    let blurred = self.blur_buffer[blur_idx] * 0.3 + sample * 0.7;
                    self.blur_buffer[blur_idx] = blurred;
                    output[i] += blurred * self.master_volume;
                }

                self.blur_position = (self.blur_position + input.len()) % blur_len;
            }
            MixMode::Stack => {
                // All voices stacked with compression
                for (i, sample) in input.iter().enumerate() {
                    let stacked = sample.tanh(); // Soft clipping
                    output[i] += stacked * self.master_volume;
                }
            }
            MixMode::Rotate => {
                // Voice rotation handled at voice allocation level
                for (i, sample) in input.iter().enumerate() {
                    output[i] += sample * self.master_volume;
                }
            }
        }
    }

    pub fn set_master_volume(&mut self, volume: f32) {
        self.master_volume = volume.clamp(0.0, 1.0);
    }
}
