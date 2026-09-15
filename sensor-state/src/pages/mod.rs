use core::fmt::Write;
use heapless::String;

#[derive(PartialEq)]
pub enum Page {
    MainMenuPage,
    RotaryPage
}

impl Page {
    pub fn header(&self) -> String<128> {
        let mut text = String::new();

        let header = match self {
            Page::MainMenuPage => "Main Menu",
            Page::RotaryPage => "Rotary",
        };

        text.push_str(header).unwrap();
        text.push_str("\0").unwrap();

        text
    }

    pub fn render(&self) -> impl Iterator<Item = Option<String<128>>> {
        let mut result: [Option<String<128>>; 7] = [const { None }; 7];

        match self {
            Page::MainMenuPage => {
                let mut text1 = String::new();
                let mut text2 = String::new();
                let mut text3 = String::new();
                let mut text4 = String::new();
                let mut text5 = String::new();
                let mut text6 = String::new();
                let mut text7 = String::new();
        
                write!(text1, "Rotary\0").unwrap();
                write!(text2, "Sound\0").unwrap();
                write!(text3, "Light\0").unwrap();
                write!(text4, "Temperature\0").unwrap();
                write!(text5, "Humidity\0").unwrap();
                write!(text6, "Pressure\0").unwrap();
                write!(text7, "Acceleration\0").unwrap();

                result[0] = Some(text1);
                result[1] = Some(text2);
                result[2] = Some(text3);
                result[3] = Some(text4);
                result[4] = Some(text5);
                result[5] = Some(text6);
                result[6] = Some(text7);

            },
            Page::RotaryPage => {},
        };

        result.into_iter()
    }
}