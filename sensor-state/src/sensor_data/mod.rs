pub mod sensor_accumulator;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SensorData {
    pub button: bool,
    pub rotary: i16,
    pub sound: i16,
    pub light: i16,

    pub temperature: f32, // Celsius
    pub humidity: f32, 
    pub pressure: f32, // Pascals

    pub acceleration: [f32; 3], // X, Y, Z
}