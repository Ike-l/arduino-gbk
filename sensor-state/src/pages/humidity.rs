use heapless::String;

use crate::{format_float, sensor_data::sensor_accumulator::{SensorAccumulator, humidity_accumulator::HumidityAccumulator}, settings::Settings};

pub fn header() -> &'static str {
    "Humidity"
}

pub fn render(
    result: &mut [Option<String<16>>; 8],
    sensor_accumulator: &SensorAccumulator,
    _settings: &Settings
) {
    let mut text1 = String::new();
    let mut text2 = String::new();
    let mut text3 = String::new();
        
    let value = sensor_accumulator.humidity.value;
    let _ = text1.push_str("val: ");
    format_float(&mut text1, value as i16);
    let _ = text1.push('%');

    let value = sensor_accumulator.humidity.max;
    if value == HumidityAccumulator::DEFAULT_MAX {
        let _ = text2.push_str("max: None");
    } else {
        let _ = text2.push_str("max: ");
        format_float(&mut text2, value as i16);
        let _ = text2.push('%');
    }
    
    let value = sensor_accumulator.humidity.min;
    if value == HumidityAccumulator::DEFAULT_MIN {
        let _ = text3.push_str("min: None");
    } else {
        let _ = text3.push_str("min: ");
        format_float(&mut text3, value as i16);
        let _ = text3.push('%');
    }
    
    result[0] = Some(text1);
    result[1] = Some(text2);
    result[2] = Some(text3);
}
