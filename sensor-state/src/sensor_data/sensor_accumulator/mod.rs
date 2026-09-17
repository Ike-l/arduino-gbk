use crate::{pages::Page, sensor_data::{SensorData, sensor_accumulator::{button_accumulator::ButtonAccumulator, light_accumulator::LightAccumulator, rotary_accumulator::RotaryAccumulator, sound_accumulator::SoundAccumulator}}};

pub mod button_accumulator;
pub mod rotary_accumulator;
pub mod sound_accumulator;
pub mod light_accumulator;

pub struct SensorAccumulator {
    pub button: ButtonAccumulator,
    pub rotary: RotaryAccumulator,
    pub sound: SoundAccumulator,
    pub light: LightAccumulator,
}

impl SensorAccumulator {
    pub const fn default() -> Self {
        Self {
            button: ButtonAccumulator::default(),
            rotary: RotaryAccumulator::default(),
            sound: SoundAccumulator::default(),
            light: LightAccumulator::default(),
        }
    }


    /// accumulates ALL new sensor data regardless if page
    pub fn track(&mut self, counted_click: bool, sensor_data: &SensorData) {
        self.button.track(counted_click, sensor_data);
        self.rotary.track(counted_click, sensor_data);
        self.sound.track(counted_click, sensor_data);
        self.light.track(counted_click, sensor_data);

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
            },
            Page::SoundPage => {
                self.sound.track(counted_click, sensor_data);
            },
            Page::LightPage => {
                self.light.track(counted_click, sensor_data);
            },
            Page::SettingsPage => {}
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
                self.sound.clear();
                self.light.clear();
            },
            Page::ButtonPage => {
                self.button.track(counted_click, sensor_data);
                self.rotary.clear();
                self.sound.clear();
                self.light.clear();
            },
            Page::RotaryPage => {
                self.button.clear();
                self.rotary.track(counted_click, sensor_data);
                self.sound.clear();
                self.light.clear();
            },
            Page::SoundPage => {
                self.button.clear();
                self.rotary.clear();
                self.sound.track(counted_click, sensor_data);
                self.light.clear();
            },
            Page::LightPage => {
                self.button.clear();
                self.rotary.clear();
                self.sound.clear();
                self.light.track(counted_click, sensor_data);
            },
            Page::SettingsPage => {
                self.button.clear();
                self.rotary.clear();
                self.sound.clear();
                self.light.clear();
            }
        }
    }
}