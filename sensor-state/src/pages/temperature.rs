use heapless::String;

use crate::{format_float, sensor_data::sensor_accumulator::{SensorAccumulator, temperature_accumulator::TemperatureAccumulator}, settings::Settings};

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

    let value = sensor_accumulator.temperature.max;
    if value == TemperatureAccumulator::DEFAULT_MAX {
        let _ = text2.push_str("max: None");
    } else {
        let _ = text2.push_str("max: ");
        format_float(&mut text2, value);
    }
    
    let value = sensor_accumulator.temperature.min;
    if value == TemperatureAccumulator::DEFAULT_MIN {
        let _ = text3.push_str("min: None");
    } else {
        let _ = text3.push_str("min: ");
        format_float(&mut text3, value);
    }
    
    result[0] = Some(text1);
    result[1] = Some(text2);
    result[2] = Some(text3);
}

