pub struct Settings {
    raw: u8
}

impl Settings {
    pub const fn default() -> Self {
        Self {
            raw: 0b0000_0011
            // raw: 0b0000_0000
            // raw: 0b0000_0010
        }
    }

    pub fn track_all(&self) -> bool {
        (self.raw & 0b0000_0001) != 0 
    }

    pub fn keep_all(&self) -> bool {
        (self.raw & 0b0000_0010) != 0 
    }

    pub fn set_track_all(&mut self, state: bool) {
        if state {
            self.raw |= 0b0000_0001;
            self.set_keep_all(true);
        } else {
            self.raw &= 0b1111_1110;
        }
    }

    pub fn set_keep_all(&mut self, state: bool) {
        if state {
            self.raw |= 0b0000_0010;
        } else {
            self.raw &= 0b1111_1101;
            self.set_track_all(false);
        }
    }
}