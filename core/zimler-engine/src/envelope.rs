use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EnvelopeShape {
    ADSR {
        attack_ms: f32,
        decay_ms: f32,
        sustain: f32,
        release_ms: f32,
    },
    AR {
        attack_ms: f32,
        release_ms: f32,
    },
    Trapezoid {
        rise_ms: f32,
        hold_ms: f32,
        fall_ms: f32,
    },
}

impl Default for EnvelopeShape {
    fn default() -> Self {
        Self::ADSR {
            attack_ms: 10.0,
            decay_ms: 100.0,
            sustain: 0.7,
            release_ms: 200.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum EnvelopeStage {
    Idle,
    Attack,
    Decay,
    Sustain,
    Release,
    Hold, // For trapezoid
}

pub struct Envelope {
    shape: EnvelopeShape,
    stage: EnvelopeStage,
    current_value: f32,
    target_value: f32,
    rate: f32,
    sample_rate: f32,
    samples_in_stage: usize,
    stage_counter: usize,
}

impl Envelope {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            shape: EnvelopeShape::default(),
            stage: EnvelopeStage::Idle,
            current_value: 0.0,
            target_value: 0.0,
            rate: 0.0,
            sample_rate,
            samples_in_stage: 0,
            stage_counter: 0,
        }
    }

    pub fn set_shape(&mut self, shape: EnvelopeShape) {
        self.shape = shape;
    }

    pub fn trigger(&mut self) {
        match self.shape {
            EnvelopeShape::ADSR { attack_ms, .. } => {
                self.stage = EnvelopeStage::Attack;
                self.target_value = 1.0;
                self.samples_in_stage = ((attack_ms / 1000.0) * self.sample_rate) as usize;
                self.rate = if self.samples_in_stage > 0 {
                    (self.target_value - self.current_value) / self.samples_in_stage as f32
                } else {
                    1.0
                };
            },
            EnvelopeShape::AR { attack_ms, .. } => {
                self.stage = EnvelopeStage::Attack;
                self.target_value = 1.0;
                self.samples_in_stage = ((attack_ms / 1000.0) * self.sample_rate) as usize;
                self.rate = if self.samples_in_stage > 0 {
                    (self.target_value - self.current_value) / self.samples_in_stage as f32
                } else {
                    1.0
                };
            },
            EnvelopeShape::Trapezoid { rise_ms, .. } => {
                self.stage = EnvelopeStage::Attack;
                self.target_value = 1.0;
                self.samples_in_stage = ((rise_ms / 1000.0) * self.sample_rate) as usize;
                self.rate = if self.samples_in_stage > 0 {
                    (self.target_value - self.current_value) / self.samples_in_stage as f32
                } else {
                    1.0
                };
            },
        }
        self.stage_counter = 0;
    }

    pub fn release(&mut self) {
        if self.stage != EnvelopeStage::Idle && self.stage != EnvelopeStage::Release {
            self.stage = EnvelopeStage::Release;
            self.target_value = 0.0;
            
            let release_ms = match self.shape {
                EnvelopeShape::ADSR { release_ms, .. } => release_ms,
                EnvelopeShape::AR { release_ms, .. } => release_ms,
                EnvelopeShape::Trapezoid { fall_ms, .. } => fall_ms,
            };
            
            self.samples_in_stage = ((release_ms / 1000.0) * self.sample_rate) as usize;
            self.rate = if self.samples_in_stage > 0 {
                -self.current_value / self.samples_in_stage as f32
            } else {
                -1.0
            };
            self.stage_counter = 0;
        }
    }

    pub fn process_sample(&mut self) {
        match self.stage {
            EnvelopeStage::Idle => {},
            EnvelopeStage::Attack => {
                self.current_value += self.rate;
                self.stage_counter += 1;
                
                if self.stage_counter >= self.samples_in_stage || self.current_value >= 1.0 {
                    self.current_value = 1.0;
                    self.advance_stage();
                }
            },
            EnvelopeStage::Decay => {
                self.current_value += self.rate;
                self.stage_counter += 1;
                
                if self.stage_counter >= self.samples_in_stage {
                    self.advance_stage();
                }
            },
            EnvelopeStage::Sustain => {
                // Hold at sustain level
            },
            EnvelopeStage::Hold => {
                self.stage_counter += 1;
                if self.stage_counter >= self.samples_in_stage {
                    self.advance_stage();
                }
            },
            EnvelopeStage::Release => {
                self.current_value += self.rate;
                self.stage_counter += 1;
                
                if self.stage_counter >= self.samples_in_stage || self.current_value <= 0.0 {
                    self.current_value = 0.0;
                    self.stage = EnvelopeStage::Idle;
                }
            },
        }
        
        self.current_value = self.current_value.clamp(0.0, 1.0);
    }

    fn advance_stage(&mut self) {
        match (&self.shape, self.stage) {
            (EnvelopeShape::ADSR { decay_ms, sustain, .. }, EnvelopeStage::Attack) => {
                self.stage = EnvelopeStage::Decay;
                self.target_value = *sustain;
                self.samples_in_stage = ((*decay_ms / 1000.0) * self.sample_rate) as usize;
                self.rate = if self.samples_in_stage > 0 {
                    (self.target_value - self.current_value) / self.samples_in_stage as f32
                } else {
                    0.0
                };
                self.stage_counter = 0;
            },
            (EnvelopeShape::ADSR { sustain, .. }, EnvelopeStage::Decay) => {
                self.stage = EnvelopeStage::Sustain;
                self.current_value = *sustain;
            },
            (EnvelopeShape::AR { .. }, EnvelopeStage::Attack) => {
                self.stage = EnvelopeStage::Sustain;
            },
            (EnvelopeShape::Trapezoid { hold_ms, fall_ms, .. }, EnvelopeStage::Attack) => {
                if *hold_ms > 0.0 {
                    self.stage = EnvelopeStage::Hold;
                    self.samples_in_stage = ((*hold_ms / 1000.0) * self.sample_rate) as usize;
                    self.stage_counter = 0;
                } else {
                    self.stage = EnvelopeStage::Release;
                    self.target_value = 0.0;
                    self.samples_in_stage = ((*fall_ms / 1000.0) * self.sample_rate) as usize;
                    self.rate = if self.samples_in_stage > 0 {
                        -self.current_value / self.samples_in_stage as f32
                    } else {
                        -1.0
                    };
                    self.stage_counter = 0;
                }
            },
            (EnvelopeShape::Trapezoid { fall_ms, .. }, EnvelopeStage::Hold) => {
                self.stage = EnvelopeStage::Release;
                self.target_value = 0.0;
                self.samples_in_stage = ((*fall_ms / 1000.0) * self.sample_rate) as usize;
                self.rate = if self.samples_in_stage > 0 {
                    -self.current_value / self.samples_in_stage as f32
                } else {
                    -1.0
                };
                self.stage_counter = 0;
            },
            _ => {},
        }
    }

    pub fn get_current_value(&self) -> f32 {
        self.current_value
    }

    pub fn is_finished(&self) -> bool {
        matches!(self.stage, EnvelopeStage::Idle)
    }
}