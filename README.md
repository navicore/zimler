# Zimler - Serge-Inspired Creative Sampler

A polyglot sampler plugin project combining Rust performance with flexible UI options.

## Architecture

Clean API separation between core engine and UI:
- **Rust Core**: High-performance audio engine with MIT/Apache2 dependencies only
- **Bevy Desktop**: Rapid prototyping with visual feedback
- **iOS AUv3**: (Future) Native Metal performance via embedded Bevy view

## Features

### Core Engine
- Multi-voice polyphonic sampler
- 1V/octave pitch shifting
- Flexible envelope shapes (ADSR, AR, Trapezoid)
- Serge-inspired mixing modes (Poly, Blur, Stack, Rotate)
- Sample-per-note or chromatic from single sample

### Bevy UI (Current)
- Real-time waveform display
- Envelope visualization
- Musical keyboard (A-K keys)
- Voice activity indicators

## Building

```bash
# Build everything
make

# Run desktop version
make run

# Development mode (faster builds)
make dev

# Run tests
make test
```

## Keyboard Controls

- **A-K**: Musical notes (C through B)
- **Space**: Load sample
- **E**: Change envelope shape

## License Policy

This project strictly uses MIT/Apache2/BSD licensed dependencies. No GPL/LGPL/MPL code.

## Dependencies

All carefully vetted for license compatibility:
- `bevy` 0.16 - MIT/Apache2
- `cpal` - Apache2 (cross-platform audio)
- `dasp` - MIT/Apache2 (sample processing)
- `rubato` - MIT (high-quality resampling)
- `midir` - MIT (MIDI support)

## Project Structure

```
zimler/
├── core/               # Shared Rust audio engine
│   ├── zimler-engine/  # Core sampler logic
│   ├── zimler-dsp/     # DSP primitives
│   └── zimler-midi/    # MIDI processing
├── desktop/            # Bevy prototype
└── ios/                # Future AUv3 plugin
```

## Next Steps

1. Implement actual sample loading from WAV files
2. Add MIDI input support
3. Create envelope editor UI
4. Implement waveform visualization
5. Add preset save/load system

## Philosophy

Inspired by Serge modulars - everything is patchable, experimental, and focused on organic sound design.