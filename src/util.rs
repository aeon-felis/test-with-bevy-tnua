pub struct SegmentedJumpInitialVelocityCalculator {
    height: f32,
    kinetic_energy: f32,
}

impl SegmentedJumpInitialVelocityCalculator {
    pub fn new(total_height: f32) -> Self {
        Self {
            height: total_height,
            kinetic_energy: 0.0,
        }
    }

    pub fn add_segment(&mut self, gravity: f32, velocity_threshold: f32) -> &mut Self {
        if self.height <= 0.0 {
            // No more height to jump
            return self;
        }

        let kinetic_energy_at_velocity_threshold = 0.5 * velocity_threshold.powi(2);

        let transferred_energy = kinetic_energy_at_velocity_threshold - self.kinetic_energy;
        if transferred_energy <= 0.0 {
            // Already faster than that velocity
            return self;
        }

        let segment_height = transferred_energy / gravity;
        if self.height < segment_height {
            // This segment will be the last
            self.kinetic_energy += self.height * gravity;
            self.height = 0.0;
        } else {
            self.kinetic_energy += transferred_energy;
            self.height -= segment_height;
        }

        self
    }

    pub fn kinetic_energy(&self) -> f32 {
        self.kinetic_energy
    }
}