use heapless::String;

use crate::{push_number, sensor_data::sensor_accumulator::SensorAccumulator, settings::Settings};

pub fn header() -> &'static str {
    "Light"
}

pub fn render(
    result: &mut [Option<String<16>>; 8],
    sensor_accumulator: &SensorAccumulator,
    _settings: &Settings
) {
    let mut text1 = String::new();
    let mut text2 = String::new();

    let _ = text1.push_str("val: ");
    let value = sensor_accumulator.light.value;
    push_number(&mut text1, value as u32);

    let _ = text2.push_str("max: ");
    let value = sensor_accumulator.light.max;
    push_number(&mut text2, value as u32);
    
    result[0] = Some(text1);
    result[1] = Some(text2);
}