pub struct Settings {
    pub track_all: bool,
    pub keep_all: bool
}

impl Settings {
    pub const fn default() -> Self {
        Self {
            track_all: true,
            keep_all: true,
        }
    }
}