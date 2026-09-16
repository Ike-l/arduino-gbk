use heapless::String;

use crate::{sensor_data::sensor_accumulator::SensorAccumulator, settings::Settings};

pub fn header() -> &'static str {
    "Settings"
}

pub fn render(
    result: &mut [Option<String<16>>; 8],
    _sensor_accumulator: &SensorAccumulator,
    settings: &Settings
) {
    let mut text1 = String::new();
    let _ = text1.push_str(if settings.track_all() { "Trk ll: T" } else { "Trk ll: F" });
    result[0] = Some(text1);

    let mut text2 = String::new();
    let _ = text2.push_str(if settings.keep_all() { "Kpp ll: T" } else { "Kpp ll: F" });
    result[1] = Some(text2);
}