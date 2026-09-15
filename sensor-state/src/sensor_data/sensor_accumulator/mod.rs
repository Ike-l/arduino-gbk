use crate::{pages::Page, sensor_data::{SensorData, sensor_accumulator::button_accumulator::ButtonAccumulator}};

pub mod button_accumulator;

pub struct SensorAccumulator {
    pub button: ButtonAccumulator
}

impl SensorAccumulator {
    pub const fn default() -> Self {
        Self {
            button: ButtonAccumulator { count: 0 }
        }
    }

    pub fn track(&mut self, _sensor_data: &SensorData) {

    }

    pub fn track_one(&mut self, _current_page: &Page, _sensor_data: &SensorData) {

    }

    pub fn keep_one(&mut self, _current_page: &Page, _sensor_data: &SensorData) {

    }
}