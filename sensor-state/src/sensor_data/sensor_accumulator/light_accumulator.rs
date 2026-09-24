use crate::sensor_data::SensorData;

pub struct LightAccumulator {
    pub value: u16,
    pub max: u16,
    pub min: u16,
}

impl LightAccumulator {
    pub const DEFAULT_MAX: u16 = 0;
    pub const DEFAULT_MIN: u16 = 9999;

    pub const fn default() -> Self {
        Self { value: 0, max: Self::DEFAULT_MAX, min: Self::DEFAULT_MIN }
    }

    pub fn track(&mut self, _counted_click: bool, sensor_data: &SensorData) {
        self.value = sensor_data.light;
        if self.value > self.max {
            self.max = self.value;
        }

        if self.value < self.min {
            self.min = self.value;
        }
    }

    pub fn clear(&mut self) {
        self.value = 0;
        self.max = Self::DEFAULT_MAX;
        self.min = Self::DEFAULT_MIN;
    }
}