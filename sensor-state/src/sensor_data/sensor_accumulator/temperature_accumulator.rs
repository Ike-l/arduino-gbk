use crate::sensor_data::SensorData;

pub struct TemperatureAccumulator {
    pub value: i16,
    pub max: i16,
    pub min: i16
}

impl TemperatureAccumulator {
    pub const DEFAULT_MIN: i16 = 9999;
    pub const DEFAULT_MAX: i16 = -9999;

    pub const fn default() -> Self {
        Self { value: 0, max: Self::DEFAULT_MAX, min: Self::DEFAULT_MIN }
    }

    pub fn track(&mut self, _counted_click: bool, sensor_data: &SensorData) {
        self.value = sensor_data.temperature;
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
        self.min = Self::DEFAULT_MIN
    }
}