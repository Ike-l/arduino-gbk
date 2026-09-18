use heapless::String;

use crate::{push_number, sensor_data::sensor_accumulator::SensorAccumulator, settings::Settings};

pub fn header() -> &'static str {
    "Temperature"
}

pub fn render(
    result: &mut [Option<String<16>>; 8],
    sensor_accumulator: &SensorAccumulator,
    _settings: &Settings
) {
    let mut text1 = String::new();
    let mut text2 = String::new();
    let mut text3 = String::new();
        
    let value = sensor_accumulator.temperature.value;
    let _ = text1.push_str("val: ");
    format_float(&mut text1, value);

    let _ = text2.push_str("max: ");
    let value = sensor_accumulator.temperature.max;
    format_float(&mut text2, value);

    let value = sensor_accumulator.temperature.min;
    if let Some(value) = value {
        let _ = text3.push_str("min: ");
        format_float(&mut text3, value);
    } else {
        let _ = text3.push_str("min: None");
    }
    
    result[0] = Some(text1);
    result[1] = Some(text2);
    result[2] = Some(text3);
}

fn format_float(mut text: &mut String<16>, mut value: i32) {
    if value < 0 {
        let _ = text.push('-');
        value = -value;
    }

    let integer = value / 10;
    let decimal = value % 10;
    push_number(&mut text, integer as u32);
    let _ = text.push('.');
    push_number(&mut text, decimal as u32);
}