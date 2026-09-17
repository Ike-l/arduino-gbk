use crate::sensor_data::SensorData;

pub struct TemperatureAccumulator {
    pub value: f32,
    pub max: f32,
    pub min: Option<f32>
}

impl TemperatureAccumulator {
    pub const fn default() -> Self {
        Self { value: 0.0, max: 0.0, min: None }
    }

    pub fn track(&mut self, _counted_click: bool, sensor_data: &SensorData) {
        self.value = sensor_data.temperature;
        if self.value > self.max {
            self.max = self.value;
        }

        if self.min.is_some_and(|v| self.value < v) {
            self.min.replace(self.value);
        }

        if self.min.is_none() {
            self.min.replace(self.value);
        }
    }

    pub fn clear(&mut self) {
        self.value = 0.0;
        self.max = 0.0;
        self.min = None
    }
}