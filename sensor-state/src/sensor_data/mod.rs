pub mod sensor_accumulator;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SensorData {
    pub button: bool,

    pub rotary: u16,
    pub sound: u16,
    pub light: u16,

    pub temperature: i16, // Celsius //  * 10

    pub humidity: u16, // * 10
    pub pressure: u16, // Pascals // * 100

    pub acceleration: [i16; 3], // X, Y, Z // * 100
}