use heapless::String;

use crate::{DrawTextCallback, SetFontCallback, current_time::EnvironmentData, display_settings::DisplaySettings, font_type::FontType, pages::Page, push_number, sensor_data::{SensorData, sensor_accumulator::SensorAccumulator}, settings::Settings};

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
        
        let mut counted_click = button_just_pressed && time_since_last_press > 200;

        // EVENTS
        let mut changed_page = false;
        if counted_click {
            let mut new_page = None;

            if sensor_data.rotary == 0 {
                new_page = Some(match self.current_page {
                    Page::MainMenuPage => Page::ButtonPage,
                    Page::ButtonPage => Page::RotaryPage,
                    Page::RotaryPage => Page::SoundPage,
                    Page::SoundPage => Page::LightPage,
                    Page::LightPage => Page::TemperaturePage,
                    Page::TemperaturePage => Page::HumidityPage,
                    Page::HumidityPage => Page::AccelerationPage,
                    Page::AccelerationPage => Page::SettingsPage,
                    Page::SettingsPage => Page::MainMenuPage,
                });
            } else if matches!(self.current_page, Page::SettingsPage) {
                if sensor_data.rotary < 128 {
                    self.settings.set_keep_all(true);
                } else if sensor_data.rotary < 256 {
                    self.settings.set_track_all(true);
                } else if sensor_data.rotary < 512 {
                    self.settings.set_keep_all(false);
                } else if sensor_data.rotary < 1024 {
                    self.settings.set_track_all(false);
                }
            }

            if let Some(new_page) = new_page {
                self.current_page = new_page;
                changed_page = true;
            }
            
            self.last_button_press_time = environment_data.current_time;
        }

        if changed_page {
            counted_click = false;
        }


        match (self.settings.keep_all(), self.settings.track_all()) {
            (_, true) => self.sensor_accumulator.track(counted_click, sensor_data),
            (true, false) => self.sensor_accumulator.track_one(counted_click, &self.current_page, sensor_data),
            (false, false) => self.sensor_accumulator.keep_one(counted_click, &self.current_page, sensor_data),
        }


        self.last_button_state = sensor_data.button;
    }

    pub fn render(
        &mut self, 
        _display_settings: &DisplaySettings,
        environment_data: &EnvironmentData,
        draw_text_cb: DrawTextCallback,
        set_font_cb: SetFontCallback,
    ) {
        let mut cursor = [0, 0];

        let header_font_height = set_font_cb(FontType::Header as u8);

        let mut page_header = String::<16>::new();
        let _ = page_header.push_str(self.current_page.header());
        let _ = page_header.push_str(" \0");
        
        let mut current_end_x = draw_text_cb(cursor[0], cursor[1], page_header.as_ptr());

        let mut timer = heapless::String::<16>::new();        
        let total_secs = environment_data.current_time / 1000;
        push_number(&mut timer, total_secs);
        let _ = timer.push_str("s \0");

        let content_font_height = set_font_cb(FontType::Contents as u8);
        current_end_x = draw_text_cb(current_end_x, cursor[1], timer.as_ptr());

        draw_text_cb(current_end_x, cursor[1], "<\0".as_ptr());

        cursor[1] += header_font_height as i16;
        for text in self.current_page.render(
            &self.sensor_accumulator,
            &self.settings
        ) {
            if let Some(mut text) = text {
                let _  = text.push('\0');
                draw_text_cb(cursor[0], cursor[1], text.as_ptr());
                cursor[1] += content_font_height as i16;
            }
        }
    }
}