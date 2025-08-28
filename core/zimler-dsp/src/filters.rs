pub struct SergeFilter {
    // Serge-inspired resonant filter
    #[allow(dead_code)]
    cutoff: f32,
    #[allow(dead_code)]
    resonance: f32,
}

impl Default for SergeFilter {
    fn default() -> Self {
        Self {
            cutoff: 1000.0,
            resonance: 0.5,
        }
    }
}

impl SergeFilter {
    pub const fn new() -> Self {
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
