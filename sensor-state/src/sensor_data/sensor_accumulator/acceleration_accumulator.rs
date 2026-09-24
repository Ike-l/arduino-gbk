use crate::sensor_data::SensorData;

pub struct AccelerationAccumulator {
    pub value: [i16; 3],
    pub min: [i16; 3],
    pub max: [i16; 3],
}

impl AccelerationAccumulator {
    pub const DEFAULT_MAX: i16 = -9999;
    pub const DEFAULT_MIN: i16 = 9999;

    pub const fn default() -> Self {
        Self { value: [0; 3], min: [Self::DEFAULT_MIN; 3], max: [Self::DEFAULT_MAX; 3] }
    }

    pub fn track(&mut self, _counted_click: bool, sensor_data: &SensorData) {
        self.value = sensor_data.acceleration;

        for i in 0..3 {
            self.max[i] = self.max[i].max(self.value[i]);
            self.min[i] = self.min[i].min(self.value[i]);
        }
    }

    pub fn clear(&mut self) {
        self.value = [0; 3];
        self.min = [Self::DEFAULT_MIN; 3];
        self.max = [Self::DEFAULT_MAX; 3];
    }
}