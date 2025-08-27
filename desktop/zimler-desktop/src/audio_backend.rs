use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}, Stream};
use zimler_engine::{ZimlerEngine, EngineConfig};
use std::sync::{Arc, Mutex};
use anyhow::Result;

pub struct AudioBackend {
    stream: Option<Stream>,
}

impl AudioBackend {
    pub fn new(engine: Arc<Mutex<ZimlerEngine>>, config: EngineConfig) -> Result<Self> {
        let host = cpal::default_host();
        let device = host.default_output_device()
            .ok_or_else(|| anyhow::anyhow!("No output device"))?;
        
        let cpal_config = cpal::StreamConfig {
            channels: config.num_channels as u16,
            sample_rate: cpal::SampleRate(config.sample_rate as u32),
            buffer_size: cpal::BufferSize::Fixed(config.block_size as u32),
        };
        
        let stream = device.build_output_stream(
            &cpal_config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let mut engine = engine.lock().unwrap();
                engine.process_block(data);
            },
            |err| eprintln!("Audio stream error: {}", err),
            None
        )?;
        
        stream.play()?;
        
        Ok(Self {
            stream: Some(stream),
        })
    }
    
    pub fn stop(&mut self) {
        if let Some(stream) = self.stream.take() {
            drop(stream);
        }
    }
}