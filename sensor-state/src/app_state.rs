use crate::{DrawTextCallback, SetFontCallback, current_time::EnvironmentData, display_settings::DisplaySettings, font_type::FontType, pages::Page, sensor_data::SensorData, settings::Settings};

pub struct AppState {
    pub current_page: Page,
    pub settings: Settings,
    
    pub value_accumulator: Option<SensorData>,

    pub last_button_state: bool,
    pub last_button_press_time: u32,
}

impl AppState {
    pub const fn default() -> Self {
        Self {
            current_page: Page::MainMenuPage,
            value_accumulator: None,
            settings: Settings {
                track_all: true,
                keep_all: true,
            },
            last_button_state: false,
            last_button_press_time: 0,
        }
    }

    pub fn tick(
        &mut self,
        sensor_data: &SensorData,
        environment_data: &EnvironmentData
    ) {
        let button_just_pressed = sensor_data.button && !self.last_button_state;

        let time_since_last_press = environment_data.current_time.wrapping_sub(self.last_button_press_time);

        if button_just_pressed && time_since_last_press > 200 {
            if self.current_page == Page::RotaryPage {
                self.current_page = Page::MainMenuPage;
            } else {
                self.current_page = Page::RotaryPage;
            }

            self.last_button_press_time = environment_data.current_time;
        }

        self.last_button_state = sensor_data.button;
        

        self.value_accumulator.replace(*sensor_data);
    }

    pub fn render(
        &mut self, 
        _display_settings: &DisplaySettings,
        draw_text_cb: DrawTextCallback,
        set_font_cb: SetFontCallback,
    ) {
        let mut cursor = [0, 0];
        
        let new_font_height = set_font_cb(FontType::Header as u8);
        let page_header = self.current_page.header();
        draw_text_cb(cursor[0], cursor[1], page_header.as_ptr());
        
        cursor[1] += new_font_height as i16;

        let new_font_height = set_font_cb(FontType::Contents as u8);
        for text in self.current_page.render() {
            if let Some(text) = text {
                draw_text_cb(cursor[0], cursor[1], text.as_ptr());
                cursor[1] += new_font_height as i16;
            }
        }
    }
}