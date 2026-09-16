use heapless::String;

use crate::{push_number, sensor_data::sensor_accumulator::SensorAccumulator};

#[derive(PartialEq)]
#[repr(u8)]
pub enum Page {
    MainMenuPage,
    ButtonPage,
    RotaryPage
}

impl Page {
    pub const fn default() -> Self {
        Self::MainMenuPage
    }

    pub fn header(&self) -> String<16> {
        let mut text = String::new();

        let header = match self {
            Page::MainMenuPage => "Main Menu",
            Page::ButtonPage => "Button",
            Page::RotaryPage => "Rotary",
        };

        text.push_str(header);

        text
    }

    pub fn render(&self, sensor_accumulator: &SensorAccumulator) -> impl Iterator<Item = Option<String<16>>> {
        let mut result: [Option<String<16>>; 8] = [const { None }; 8];

        match self {
            Page::MainMenuPage => {
                let mut text1 = String::new();
                let mut text2 = String::new();
                let mut text3 = String::new();
                let mut text4 = String::new();
                let mut text5 = String::new();
                // let mut text6 = String::new();
                // let mut text7 = String::new();
                // let mut text8 = String::new();
        
                let _ = text1.push_str("Button");
                let _ = text2.push_str("Rotary");
                let _ = text3.push_str("Sound");
                let _ = text4.push_str("Light");
                let _ = text5.push_str("Humidity");
                // let _ = text6.push_str("Temperature\0");
                // let _ = text7.push_str("Pressure\0");
                // let _ = text8.push_str("Acceleration\0");

                result[0] = Some(text1);
                result[1] = Some(text2);
                result[2] = Some(text3);
                result[3] = Some(text4);
                result[4] = Some(text5);
                // result[5] = Some(text6);
                // result[6] = Some(text7);
                // result[7] = Some(text8);

            },
            Page::ButtonPage => {
                let mut text1 = String::new();
                let _ = text1.push_str("count: ");
                
                let count = sensor_accumulator.button.count;

                assert!(count < 1_000);

                // Safety
                // panics
                unsafe { push_number::<16, 4>(&mut text1, count) };
                
                result[0] = Some(text1);
            },
            Page::RotaryPage => {
                let mut text1 = String::new();
                let _ = text1.push_str("value: ");
                
                let value = sensor_accumulator.rotary.value;

                assert!(value < 10_000);
                assert!(value >= 0);

                // Safety
                // panics
                unsafe { push_number::<16, 5>(&mut text1, value as u32) };
                
                result[0] = Some(text1);
            },
        };

        result.into_iter()
    }
}