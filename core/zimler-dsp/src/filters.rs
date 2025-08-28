pub struct SergeFilter {
    // Serge-inspired resonant filter
    cutoff: f32,
    resonance: f32,
}

impl SergeFilter {
    pub fn new() -> Self {
        Self {
            cutoff: 1000.0,
            resonance: 0.5,
        }
    }

    pub fn process(&mut self, input: f32) -> f32 {
        // TODO: Implement Serge-style filter
        input
    }
}
