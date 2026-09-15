#[repr(C)]
#[derive(Clone, Copy)]
pub struct DisplaySettings {
    pub font_height: i8,
    pub screen_width: u8,
    pub screen_height: u8,
}