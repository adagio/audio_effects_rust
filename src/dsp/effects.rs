pub struct LowpassFilter {
    cutoff_frequency_hz: f32, 
    prev_output: f32,
}

impl LowpassFilter {
    pub fn new(cutoff_frequency_hz: f32) -> Self { 
        Self {
            cutoff_frequency_hz,
            prev_output: 0.0, 
        }
    }

    pub fn process(&mut self, sample: f32) -> f32 {
        const SAMPLE_RATE: f32 = 44100.0; 
        let dt = 1.0 / SAMPLE_RATE;
        let rc = 1.0 / (self.cutoff_frequency_hz * 2.0 * std::f32::consts::PI);
        let alpha = dt / (rc + dt);
        let output = alpha * sample + (1.0 - alpha) * self.prev_output;
        self.prev_output = output;
        output
    }
}

pub struct Reverb {
    room_size: f32,
    damping: f32,
    wet_level: f32,
    dry_level: f32,
}

impl Reverb {
    pub fn new(room_size: f32, damping: f32, wet_level: f32, dry_level: f32) -> Self {
        Self {
            room_size,
            damping,
            wet_level,
            dry_level,
        }
    }

    pub fn process(&mut self, sample: f32) -> f32 {
        let dry_signal = sample * self.dry_level;
        let wet_signal = 0.0; 
        dry_signal + wet_signal
    }
}
