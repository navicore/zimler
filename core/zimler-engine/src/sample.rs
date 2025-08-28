use anyhow::{anyhow, Result};
use hound;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Sample {
    pub data: Vec<f32>,
    pub sample_rate: f32,
    pub channels: usize,
    pub root_note: Option<u8>,
}

impl Sample {
    pub fn new(data: Vec<f32>, sample_rate: f32, channels: usize) -> Self {
        Self {
            data,
            sample_rate,
            channels,
            root_note: Some(60), // Middle C default
        }
    }

    pub fn duration_ms(&self) -> f32 {
        (self.data.len() as f32 / self.channels as f32 / self.sample_rate) * 1000.0
    }
}

pub struct SampleBank {
    samples: HashMap<usize, Sample>,
    current_mapping: SampleMapping,
}

#[derive(Debug, Clone)]
pub enum SampleMapping {
    ChromaticSingle { slot: usize },
    MultiSample(HashMap<u8, usize>),
    Velocity(Vec<(f32, usize)>),
    RoundRobin { slots: Vec<usize>, current: usize },
}

impl Default for SampleBank {
    fn default() -> Self {
        Self::new()
    }
}

impl SampleBank {
    pub fn new() -> Self {
        Self {
            samples: HashMap::new(),
            current_mapping: SampleMapping::ChromaticSingle { slot: 0 },
        }
    }

    pub fn load_sample(&mut self, slot: usize, path: &str) -> Result<()> {
        let sample = Self::load_wav(path)?;
        self.samples.insert(slot, sample);
        Ok(())
    }

    fn load_wav(path: &str) -> Result<Sample> {
        let path = Path::new(path);
        if !path.exists() {
            return Err(anyhow!("File not found: {:?}", path));
        }

        let mut reader = hound::WavReader::open(path)?;
        let spec = reader.spec();

        // Convert to f32 samples
        let mut data = Vec::new();
        match spec.sample_format {
            hound::SampleFormat::Float => {
                for sample in reader.samples::<f32>() {
                    data.push(sample?);
                }
            }
            hound::SampleFormat::Int => {
                let max_val = (1 << (spec.bits_per_sample - 1)) as f32;
                for sample in reader.samples::<i32>() {
                    data.push(sample? as f32 / max_val);
                }
            }
        }

        // If stereo, interleave the channels
        let channels = spec.channels as usize;
        let sample_rate = spec.sample_rate as f32;

        Ok(Sample::new(data, sample_rate, channels))
    }

    pub fn get_sample_for_note(&mut self, note: u8, velocity: f32) -> Option<&Sample> {
        match &mut self.current_mapping {
            SampleMapping::ChromaticSingle { slot } => self.samples.get(slot),
            SampleMapping::MultiSample(map) => {
                map.get(&note).and_then(|slot| self.samples.get(slot))
            }
            SampleMapping::Velocity(layers) => layers
                .iter()
                .find(|(thresh, _)| velocity <= *thresh)
                .and_then(|(_, slot)| self.samples.get(slot)),
            SampleMapping::RoundRobin { slots, current } => {
                let slot = slots.get(*current)?;
                *current = (*current + 1) % slots.len();
                self.samples.get(slot)
            }
        }
    }
}
