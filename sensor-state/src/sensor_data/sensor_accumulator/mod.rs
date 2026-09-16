use crate::{pages::Page, sensor_data::{SensorData, sensor_accumulator::{button_accumulator::ButtonAccumulator, rotary_accumulator::RotaryAccumulator}}};

pub mod button_accumulator;
pub mod rotary_accumulator;

pub struct SensorAccumulator {
    pub button: ButtonAccumulator,
    pub rotary: RotaryAccumulator,
}

impl SensorAccumulator {
    pub const fn default() -> Self {
        Self {
            button: ButtonAccumulator { count: 0 },
            rotary: RotaryAccumulator { value: 0 }
        }
    }


    /// accumulates ALL new sensor data regardless if page
    pub fn track(&mut self, counted_click: bool, sensor_data: &SensorData) {
        self.button.track(counted_click, sensor_data);
        self.rotary.track(counted_click, sensor_data);

        // sensor_data.acceleration
        // sensor_data.humidity
        // sensor_data.light
        // sensor_data.pressure
        // sensor_data.rotary
        // sensor_data.sound
        // sensor_data.temperature
    }

    /// accumulates only the new sensor data for the current page
    pub fn track_one(&mut self, counted_click: bool, current_page: &Page, sensor_data: &SensorData) {
        match current_page {
            Page::MainMenuPage => {},
            Page::ButtonPage => {
                self.button.track(counted_click, sensor_data);
            },
            Page::RotaryPage => {
                self.rotary.track(counted_click, sensor_data);
            }
        }
    }

    /// accumulates only the new sensor data for the current page
    /// 
    /// removing all other accumulated data
    pub fn keep_one(&mut self, counted_click: bool, current_page: &Page, sensor_data: &SensorData) {
        match current_page {
            Page::MainMenuPage => {
                self.button.clear();
                self.rotary.clear();
            },
            Page::ButtonPage => {
                self.button.track(counted_click, sensor_data);
                self.rotary.clear();
            },
            Page::RotaryPage => {
                self.rotary.track(counted_click, sensor_data);
                self.rotary.value = sensor_data.rotary;
                self.button.clear();
            }
        }
    }
}