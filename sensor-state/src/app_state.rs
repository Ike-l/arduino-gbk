use crate::{DrawTextCallback, SetFontCallback, current_time::EnvironmentData, display_settings::DisplaySettings, font_type::FontType, pages::Page, sensor_data::{SensorData, sensor_accumulator::SensorAccumulator}, settings::Settings};

pub struct AppState {
    pub current_page: Page,
    pub settings: Settings,
    
    pub sensor_accumulator: SensorAccumulator,

    pub last_button_state: bool,
    pub last_button_press_time: u32,
}

impl AppState {
    pub const fn default() -> Self {
        Self {
            current_page: Page::default(),
            sensor_accumulator: SensorAccumulator::default(),
            settings: Settings::default(),
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
            let selected_page = if self.current_page == Page::MainMenuPage { Page::RotaryPage } else { Page::MainMenuPage }; 
            self.current_page = selected_page;

            self.last_button_press_time = environment_data.current_time;
        }

        self.last_button_state = sensor_data.button;
        
        match (self.settings.keep_all, self.settings.track_all) {
            (_, true) => {
                // accumulates ALL new sensor data regardless if page
                self.sensor_accumulator.track(sensor_data);
            },
            (true, false) => {
                // accumulates only the new sensor data for the current page
                self.sensor_accumulator.track_one(&self.current_page, sensor_data);
            },
            (false, false) => {
                // accumulates only the new sensor data for the current page
                // removing all other accumulated data
                self.sensor_accumulator.keep_one(&self.current_page, sensor_data);
            },
        }
    }

    pub fn render(
        &mut self, 
        _display_settings: &DisplaySettings,
        environment_data: &EnvironmentData,
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