# Zimler - Serge-Inspired Creative Sampler

## Project Philosophy
- **Iterative Development**: We'll try different approaches until it feels right
- **Modular Architecture**: Like Serge, everything is a module that can be patched
- **Performance First**: Real-time audio demands zero-allocation hot paths
- **CLI-Only Build**: No Xcode IDE, pure command-line workflow

## Monorepo Structure

```
zimler/
├── Cargo.toml                 # Workspace root
├── Makefile                   # Top-level orchestration
├── .cargo/
│   └── config.toml            # Cross-compilation settings
│
├── core/                      # Shared Rust audio engine
│   ├── zimler-engine/         # Core sampler logic
│   ├── zimler-dsp/            # DSP primitives (envelopes, filters)
│   ├── zimler-midi/           # MIDI processing
│   └── zimler-ffi/            # C ABI for iOS integration
│
├── desktop/                   # Bevy prototype
│   └── zimler-desktop/        # Bevy UI + engine integration
│
├── ios/                       # iOS AUv3 plugin
│   ├── ZimlerAU/              # Swift/ObjC wrapper
│   ├── xcconfig/              # Build configurations
│   └── build-ios.sh           # iOS build automation
│
├── assets/                    # Shared resources
│   ├── samples/               # Test samples
│   └── presets/               # Default patches
│
└── tools/                     # Build utilities
    ├── cargo-make/            # Rust build tasks
    └── scripts/               # Helper scripts
```

## Core Audio Engine Design

### Key Modules

```rust
// zimler-engine/src/lib.rs

pub trait SamplePlayer {
    fn trigger(&mut self, note: u8, velocity: f32);
    fn process(&mut self, output: &mut [f32]);
}

pub trait Envelope {
    fn trigger(&mut self);
    fn release(&mut self);
    fn process(&mut self, samples: &mut [f32]);
}

pub struct VoicePool {
    voices: Vec<Voice>,
    envelope_shapes: EnvelopeBank,
    mixing_mode: MixMode,
}

pub enum MixMode {
    Poly,           // Traditional polyphonic
    Blur(f32),      // Serge-style overlapping with crossfade
    Stack,          // All voices always active
    Rotate,         // Round-robin voice stealing
}
```

### Envelope Shapes (Serge-Inspired)

```rust
pub enum EnvelopeShape {
    Linear { attack_ms: f32, decay_ms: f32, sustain: f32, release_ms: f32 },
    Exponential { curvature: f32, stages: Vec<Stage> },
    Trapezoid { rise_ms: f32, hold_ms: f32, fall_ms: f32 },
    Variable { control_points: Vec<(f32, f32)> },  // For drawing custom shapes
    Cycling { period_ms: f32, shape: Box<EnvelopeShape> },  // LFO-like
}
```

### Sample Management

```rust
pub enum SampleMapping {
    ChromaticSingle { root_note: u8, sample: Sample },     // 1V/oct from single sample
    MultiSample(BTreeMap<u8, Sample>),                     // Sample per note/range
    Velocity(Vec<(f32, Sample)>),                          // Velocity layers
    RoundRobin(Vec<Sample>),                               // Alternate samples
}
```

## Build System

### Root Makefile

```makefile
.PHONY: all desktop ios test clean

all: desktop

desktop:
	cargo build --release -p zimler-desktop

ios: ios-libs
	cd ios && ./build-ios.sh

ios-libs:
	cargo lipo --release -p zimler-ffi
	
test:
	cargo test --workspace
	
run: desktop
	./target/release/zimler-desktop

clean:
	cargo clean
	rm -rf ios/build
```

### Cargo.toml Workspace

```toml
[workspace]
members = [
    "core/zimler-engine",
    "core/zimler-dsp",
    "core/zimler-midi",
    "core/zimler-ffi",
    "desktop/zimler-desktop",
]

[workspace.package]
version = "0.1.0"
authors = ["Your Name"]
edition = "2021"

[workspace.dependencies]
# Audio
cpal = "0.15"
dasp = "0.11"
fundsp = "0.17"

# MIDI
midir = "0.9"

# Desktop UI
bevy = "0.14"
bevy_egui = "0.28"

# FFI
safer-ffi = "0.1"

[profile.release]
lto = "thin"
opt-level = 3
codegen-units = 1
```

## Development Phases

### Phase 1: Core Engine (Week 1-2)
- [ ] Basic sample playback
- [ ] ADSR envelope
- [ ] MIDI note handling
- [ ] Voice allocation

### Phase 2: Bevy Prototype (Week 3-4)
- [ ] Waveform display
- [ ] Envelope editor
- [ ] Sample mapping UI
- [ ] MIDI learn

### Phase 3: Advanced Features (Week 5-6)
- [ ] Custom envelope shapes
- [ ] Voice blur/mixing modes
- [ ] Modulation routing
- [ ] Preset system

### Phase 4: iOS Port (Week 7-8)
- [ ] FFI bindings
- [ ] AUv3 wrapper
- [ ] SwiftUI interface
- [ ] App Store prep

## Key Design Decisions

1. **Sample Loading**: Memory-mapped files for efficiency
2. **Voice Architecture**: Pre-allocated pool, no allocations during playback
3. **Envelope Processing**: SIMD when available (using `wide` crate)
4. **MIDI**: Lock-free ringbuffer between MIDI and audio threads
5. **State Management**: Atomic parameters for thread-safe automation

## Testing Strategy

- Unit tests for DSP algorithms
- Integration tests with test samples
- Bevy prototype for rapid iteration
- TestFlight for iOS beta testing

## Next Steps

1. Set up the monorepo structure
2. Implement basic sample playback in Rust
3. Create simple Bevy UI to test the engine
4. Iterate on envelope shapes based on your Serge experience