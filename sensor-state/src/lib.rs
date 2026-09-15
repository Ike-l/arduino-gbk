#![no_std]
use core::panic::PanicInfo;

pub mod sensor_data;
pub mod pages;
pub mod settings;
pub mod app_state;
pub mod display_settings;
pub mod current_time;
pub mod font_type;

use sensor_data::SensorData;
use app_state::AppState;
use display_settings::DisplaySettings;

use crate::current_time::EnvironmentData;

type DrawTextCallback = extern "C" fn(x: i16, y: i16, text: *const u8);
type SetFontCallback = extern "C" fn(font_type: u8) -> i8;
unsafe extern "C" {
    fn panic_blink() -> !; 
}

static mut APP_STATE: AppState = AppState::default();

#[unsafe(no_mangle)]
pub extern "C" fn render(
    display_settings: *const DisplaySettings,
    draw_text_cb: DrawTextCallback,
    set_font_cb: SetFontCallback,
) {
    let state = unsafe { &mut *(&raw mut APP_STATE) };

    let display_settings = unsafe { &*display_settings };

    state.render(
        display_settings,
        draw_text_cb,
        set_font_cb
    );
}

#[unsafe(no_mangle)]
pub extern "C" fn tick(
    sensor_data: *const SensorData, 
    environment_data: *const EnvironmentData,
) {
    let state = unsafe { &mut *(&raw mut APP_STATE) };
    
    let sensor_data = unsafe { &*sensor_data };
    let environment_data = unsafe { &*environment_data };

    state.tick(
        sensor_data,
        environment_data
    );
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        panic_blink(); 
    }
}