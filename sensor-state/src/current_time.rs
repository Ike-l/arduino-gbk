// remember to not use data here that could if changed could mess up a display page
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EnvironmentData {
    pub current_time: u32,
}