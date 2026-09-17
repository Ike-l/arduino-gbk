use crate::sensor_data::SensorData;

pub struct RotaryAccumulator {
    pub value: i16
}

impl RotaryAccumulator {
    pub const fn default() -> Self {
        Self { value: 0 }
    }

    pub fn track(&mut self, _counted_click: bool, sensor_data: &SensorData) {
        self.value = sensor_data.rotary;
    }

    pub fn clear(&mut self) {
        self.value = 0;
    }
}