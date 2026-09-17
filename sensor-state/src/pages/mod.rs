use heapless::String;

use crate::{sensor_data::sensor_accumulator::SensorAccumulator, settings::Settings};

pub mod main_menu;
pub mod button;
pub mod rotary;
pub mod sound;
pub mod light;
pub mod settings;

#[derive(PartialEq)]
#[repr(u8)]
pub enum Page {
    MainMenuPage,
    ButtonPage,
    RotaryPage,
    SoundPage,
    LightPage,
    SettingsPage
}

impl Page {
    pub const fn default() -> Self {
        Self::MainMenuPage
    }

    pub fn header(&self) -> String<16> {
        let mut text = String::new();

        let header = match self {
            Page::MainMenuPage => main_menu::header(),
            Page::ButtonPage => button::header(),
            Page::RotaryPage => rotary::header(),
            Page::SoundPage => sound::header(),
            Page::LightPage => light::header(),
            Page::SettingsPage => settings::header(),
        };

        text.push_str(header);

        text
    }

    pub fn render(
        &self, 
        sensor_accumulator: &SensorAccumulator,
        settings: &Settings
    ) -> impl Iterator<Item = Option<String<16>>> {
        let mut result: [Option<String<16>>; 8] = [const { None }; 8];

        match self {
            Page::MainMenuPage => {
                main_menu::render(&mut result, sensor_accumulator, settings)
            },
            Page::ButtonPage => {
                button::render(&mut result, sensor_accumulator, settings);
            },
            Page::RotaryPage => {
                rotary::render(&mut result, sensor_accumulator, settings);
            },
            Page::SoundPage => {
                sound::render(&mut result, sensor_accumulator, settings);
            },
            Page::LightPage => {
                light::render(&mut result, sensor_accumulator, settings);
            },
            Page::SettingsPage => {
                settings::render(&mut result, sensor_accumulator, settings);
            }
        };

        result.into_iter()
    }
}