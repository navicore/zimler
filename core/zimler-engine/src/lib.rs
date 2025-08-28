#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_const_for_fn)]

use parking_lot::RwLock;
use std::sync::Arc;

pub mod api;
pub mod envelope;
pub mod mixer;
pub mod sample;
pub mod voice;

pub use api::*;
pub use envelope::*;
pub use mixer::*;
pub use sample::*;
pub use voice::*;

#[derive(Debug, Clone)]
pub struct EngineConfig {
    pub sample_rate: f32,
    pub block_size: usize,
    pub num_voices: usize,
    pub num_channels: usize,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            sample_rate: 48000.0,
            block_size: 256,
            num_voices: 16,
            num_channels: 2,
        }
    }
}

pub struct ZimlerEngine {
    #[allow(dead_code)]
    config: EngineConfig,
    voices: Vec<Voice>,
    mixer: Mixer,
    sample_bank: Arc<RwLock<SampleBank>>,
    engine_state: Arc<RwLock<EngineState>>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct EngineState {
    pub active_voices: usize,
    pub cpu_load: f32,
    pub current_preset: Option<String>,
}

impl ZimlerEngine {
    pub fn new(config: EngineConfig) -> Self {
        let voices = (0..config.num_voices)
            .map(|_| Voice::new(config.sample_rate))
            .collect();

        Self {
            config,
            voices,
            mixer: Mixer::new(),
            sample_bank: Arc::new(RwLock::new(SampleBank::new())),
            engine_state: Arc::new(RwLock::new(EngineState::default())),
        }
    }

    pub fn process_block(&mut self, output: &mut [f32]) {
        output.fill(0.0);

        let mut temp_buffer = vec![0.0; output.len()];
        let mut active_count = 0;

        for voice in &mut self.voices {
            if voice.is_active() {
                voice.process_block(&mut temp_buffer);
                active_count += 1;
            }
        }

        self.mixer.mix(&temp_buffer, output);

        let mut state = self.engine_state.write();
        state.active_voices = active_count;
    }

    pub fn get_api_handle(&self) -> EngineHandle {
        EngineHandle {
            sample_bank: Arc::clone(&self.sample_bank),
            engine_state: Arc::clone(&self.engine_state),
        }
    }
}
