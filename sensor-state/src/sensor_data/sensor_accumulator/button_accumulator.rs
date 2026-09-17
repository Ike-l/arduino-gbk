use crate::sensor_data::SensorData;

pub struct ButtonAccumulator {
    pub count: u32,
}

impl ButtonAccumulator {
    pub const fn default() -> Self {
        Self { count: 0 }
    }

    pub fn track(&mut self, counted_click: bool, _sensor_data: &SensorData) {
        if counted_click {
            self.count += 1;
        }
    }

    pub fn clear(&mut self) {
        self.count = 0;
    }
}