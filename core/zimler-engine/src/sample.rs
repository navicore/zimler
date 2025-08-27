use anyhow::Result;
use std::collections::HashMap;

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

impl SampleBank {
    pub fn new() -> Self {
        Self {
            samples: HashMap::new(),
            current_mapping: SampleMapping::ChromaticSingle { slot: 0 },
        }
    }

    pub fn load_sample(&mut self, slot: usize, _path: &str) -> Result<()> {
        // For now, create a test sample
        let test_sample = Sample::new(
            vec![0.0; 48000], 
            48000.0, 
            1
        );
        
        self.samples.insert(slot, test_sample);
        Ok(())
    }

    pub fn get_sample_for_note(&mut self, note: u8, velocity: f32) -> Option<&Sample> {
        match &mut self.current_mapping {
            SampleMapping::ChromaticSingle { slot } => {
                self.samples.get(slot)
            },
            SampleMapping::MultiSample(map) => {
                map.get(&note).and_then(|slot| self.samples.get(slot))
            },
            SampleMapping::Velocity(layers) => {
                layers.iter()
                    .find(|(thresh, _)| velocity <= *thresh)
                    .and_then(|(_, slot)| self.samples.get(slot))
            },
            SampleMapping::RoundRobin { slots, current } => {
                let slot = slots.get(*current)?;
                *current = (*current + 1) % slots.len();
                self.samples.get(slot)
            }
        }
    }
}