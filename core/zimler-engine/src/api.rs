use crate::{EngineState, EnvelopeShape, SampleBank};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
pub struct EngineHandle {
    pub sample_bank: Arc<RwLock<SampleBank>>,
    pub engine_state: Arc<RwLock<EngineState>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EngineCommand {
    LoadSample { slot: usize, path: String },
    TriggerNote { note: u8, velocity: f32 },
    ReleaseNote { note: u8 },
    SetEnvelope { envelope: EnvelopeShape },
    SetMixMode { mode: MixMode },
    SetParameter { param: Parameter, value: f32 },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Parameter {
    MasterVolume,
    VoiceBlur,
    EnvelopeAttack,
    EnvelopeDecay,
    EnvelopeSustain,
    EnvelopeRelease,
    SampleStartOffset,
    SampleEndOffset,
    PitchBendRange,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MixMode {
    Poly,
    Blur { crossfade_ms: f32 },
    Stack,
    Rotate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineQuery {
    pub query_type: QueryType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueryType {
    GetState,
    GetSampleList,
    GetCurrentPreset,
    GetWaveform { voice_index: usize },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineResponse {
    pub success: bool,
    pub data: Option<ResponseData>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseData {
    State(EngineState),
    SampleList(Vec<SampleInfo>),
    Preset(String),
    Waveform(Vec<f32>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SampleInfo {
    pub slot: usize,
    pub name: String,
    pub duration_ms: f32,
    pub sample_rate: f32,
    pub channels: usize,
}

impl EngineHandle {
    pub fn send_command(&self, command: EngineCommand) -> Result<(), String> {
        if let EngineCommand::LoadSample { slot, path } = command {
            let mut bank = self.sample_bank.write();
            bank.load_sample(slot, &path).map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub fn query(&self, query: EngineQuery) -> EngineResponse {
        match query.query_type {
            QueryType::GetState => {
                let state = self.engine_state.read().clone();
                EngineResponse {
                    success: true,
                    data: Some(ResponseData::State(state)),
                    error: None,
                }
            }
            _ => EngineResponse {
                success: false,
                data: None,
                error: Some("Not implemented".to_string()),
            },
        }
    }
}
