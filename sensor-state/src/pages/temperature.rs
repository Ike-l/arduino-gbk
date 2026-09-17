use heapless::String;

use crate::{push_float, sensor_data::sensor_accumulator::SensorAccumulator, settings::Settings};

pub fn header() -> &'static str {
    "Temperature"
}

pub fn render(
    result: &mut [Option<String<16>>; 8],
    sensor_accumulator: &SensorAccumulator,
    _settings: &Settings
) {
    let mut text1 = String::new();
    let _ = text1.push_str("val: ");
    let value = sensor_accumulator.temperature.value;
    push_float(&mut text1, value);

    let mut text2 = String::new();
    let _ = text2.push_str("max: ");
    let value = sensor_accumulator.temperature.max;
    push_float(&mut text2, value);

    let mut text3 = String::new();
    let value = sensor_accumulator.temperature.min;
    
    if let Some(val) = value {
        let _ = text3.push_str("min: ");
        push_float(&mut text3, val);
    } else {
        let _ = text3.push_str("min: None");
    }
    
    result[0] = Some(text1);
    result[1] = Some(text2);
    result[2] = Some(text3);
}