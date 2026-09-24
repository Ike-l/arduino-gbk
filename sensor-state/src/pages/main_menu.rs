use heapless::String;

use crate::{sensor_data::sensor_accumulator::SensorAccumulator, settings::Settings};

pub fn header() -> &'static str {
    "Main Menu"
}

pub fn render(
    result: &mut [Option<String<16>>; 8],
    _sensor_accumulator: &SensorAccumulator,
    _settings: &Settings
) {
    let mut text1 = String::new();
    let mut text2 = String::new();
    let mut text3 = String::new();
    let mut text4 = String::new();
    let mut text5 = String::new();
    let mut text6 = String::new();
    let mut text7 = String::new();
    // let mut text8 = String::new();

    let _ = text1.push_str(super::button::header());
    let _ = text2.push_str(super::rotary::header());
    let _ = text3.push_str(super::sound::header());
    let _ = text4.push_str(super::light::header());
    let _ = text5.push_str(super::temperature::header());
    let _ = text6.push_str(super::humidity::header());
    let _ = text7.push_str(super::settings::header());
    // let _ = text7.push_str("Pressure\0");
    // let _ = text8.push_str("Acceleration\0");

    result[0] = Some(text1);
    result[1] = Some(text2);
    result[2] = Some(text3);
    result[3] = Some(text4);
    result[4] = Some(text5);
    result[5] = Some(text6);
    result[6] = Some(text7);
    // result[7] = Some(text8);
}