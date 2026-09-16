use heapless::String;

use crate::{push_number, sensor_data::sensor_accumulator::SensorAccumulator, settings::Settings};

pub fn header() -> &'static str {
    "Button"
}
pub fn render(
    result: &mut [Option<String<16>>; 8],
    sensor_accumulator: &SensorAccumulator,
    _settings: &Settings
) {
    let mut text1 = String::new();
    let _ = text1.push_str("cnt: ");
    
    let count = sensor_accumulator.button.count;

    assert!(count < 1_000);

    // Safety
    // panics
    unsafe { push_number::<16, 4>(&mut text1, count) };
    
    result[0] = Some(text1);
}