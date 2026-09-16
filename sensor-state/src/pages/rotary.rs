use heapless::String;

use crate::{push_number, sensor_data::sensor_accumulator::SensorAccumulator, settings::Settings};

pub fn header() -> &'static str {
    "Rotary"
}

pub fn render(
    result: &mut [Option<String<16>>; 8],
    sensor_accumulator: &SensorAccumulator,
    _settings: &Settings
) {
    let mut text1 = String::new();
    let _ = text1.push_str("val: ");
    
    let value = sensor_accumulator.rotary.value;

    assert!(value < 10_000);
    assert!(value >= 0);

    // Safety
    // panics
    unsafe { push_number::<16, 5>(&mut text1, value as u32) };
    
    result[0] = Some(text1);
}