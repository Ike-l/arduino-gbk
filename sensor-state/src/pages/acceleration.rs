use heapless::String;

use crate::{format_float, sensor_data::sensor_accumulator::SensorAccumulator, settings::Settings};

pub fn header() -> &'static str {
    "Acceleration"
}

pub fn render(
    result: &mut [Option<String<16>>; 8],
    sensor_accumulator: &SensorAccumulator,
    _settings: &Settings
) {
    let mut text1 = String::new();
    let mut text2 = String::new();
    let mut text3 = String::new();
    let mut text4 = String::new();
    let mut text5 = String::new();
    let mut text6 = String::new();
        
    let value = sensor_accumulator.acceleration.value;
    let _ = text1.push_str("val:");
    format_float(&mut text2, value[0]);
    let _ = text2.push_str(",");
    format_float(&mut text2, value[1]);
    let _ = text2.push_str(",");
    format_float(&mut text2, value[2]);

    let value = sensor_accumulator.acceleration.max;
    let _ = text3.push_str("max:");
    format_float(&mut text4, value[0]);
    let _ = text4.push_str(",");
    format_float(&mut text4, value[1]);
    let _ = text4.push_str(",");
    format_float(&mut text4, value[2]);

    let value = sensor_accumulator.acceleration.min;
    let _ = text5.push_str("min:");
    format_float(&mut text6, value[0]);
    let _ = text6.push_str(",");
    format_float(&mut text6, value[1]);
    let _ = text6.push_str(",");
    format_float(&mut text6, value[2]);

    result[0] = Some(text1);
    result[1] = Some(text2);
    result[2] = Some(text3);
    result[3] = Some(text4);
    result[4] = Some(text5);
    result[5] = Some(text6);
}
