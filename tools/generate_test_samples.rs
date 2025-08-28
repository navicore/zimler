#!/usr/bin/env cargo +nightly -Zscript

//! ```cargo
//! [dependencies]
//! hound = "3.5"
//! ```

use hound;
use std::f32::consts::PI;

fn main() {
    // Create assets/samples directory
    std::fs::create_dir_all("assets/samples").unwrap();
    
    // Generate a sine wave at 440Hz (A4)
    generate_sine("assets/samples/sine_440.wav", 440.0, 1.0, 48000);
    
    // Generate a short click/blip
    generate_click("assets/samples/click.wav", 48000);
    
    // Generate a sweep
    generate_sweep("assets/samples/sweep.wav", 100.0, 2000.0, 1.0, 48000);
    
    println!("Test samples generated in assets/samples/");
}

fn generate_sine(path: &str, freq: f32, duration: f32, sample_rate: u32) {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    
    let mut writer = hound::WavWriter::create(path, spec).unwrap();
    let num_samples = (duration * sample_rate as f32) as usize;
    
    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let sample = (t * freq * 2.0 * PI).sin();
        let amplitude = i16::MAX as f32;
        writer.write_sample((sample * amplitude * 0.5) as i16).unwrap();
    }
    
    writer.finalize().unwrap();
    println!("Generated {}", path);
}

fn generate_click(path: &str, sample_rate: u32) {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    
    let mut writer = hound::WavWriter::create(path, spec).unwrap();
    let num_samples = (0.01 * sample_rate as f32) as usize; // 10ms click
    
    for i in 0..num_samples {
        let envelope = 1.0 - (i as f32 / num_samples as f32);
        let sample = envelope * (i as f32 * 0.1).sin();
        let amplitude = i16::MAX as f32;
        writer.write_sample((sample * amplitude) as i16).unwrap();
    }
    
    // Add silence
    for _ in 0..sample_rate / 10 {
        writer.write_sample(0i16).unwrap();
    }
    
    writer.finalize().unwrap();
    println!("Generated {}", path);
}

fn generate_sweep(path: &str, start_freq: f32, end_freq: f32, duration: f32, sample_rate: u32) {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    
    let mut writer = hound::WavWriter::create(path, spec).unwrap();
    let num_samples = (duration * sample_rate as f32) as usize;
    
    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let progress = t / duration;
        let freq = start_freq + (end_freq - start_freq) * progress;
        
        // Integrate phase for smooth frequency change
        let phase = 2.0 * PI * (start_freq * t + 0.5 * (end_freq - start_freq) * t * t / duration);
        let sample = phase.sin();
        
        let amplitude = i16::MAX as f32;
        writer.write_sample((sample * amplitude * 0.5) as i16).unwrap();
    }
    
    writer.finalize().unwrap();
    println!("Generated {}", path);
}