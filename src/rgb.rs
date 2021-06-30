#[derive(Debug, Clone)]
pub struct RGB(pub i32, pub i32, pub i32);

impl RGB {
    pub fn new(r: u8, g: u8, b: u8) -> RGB {
        RGB(r as i32, g as i32, b as i32)
    }

    pub fn r(&self) -> i32 {
        self.0
    }
    pub fn g(&self) -> i32 {
        self.1
    }
    pub fn b(&self) -> i32 {
        self.2
    }
}

impl VecColor for RGB {
    fn new_hex(val: u32) -> RGB {
        let r = (val & 0x00FF0000) >> 16;
        let g = (val & 0x0000FF00) >> 8;
        let b = val & 0x000000FF;

        RGB(r as i32, g as i32, b as i32)
    }
}

#[derive(Debug, Clone)]
pub struct ARGB(pub i32, pub i32, pub i32, pub i32);

impl ARGB {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> ARGB {
        ARGB(a as i32, r as i32, g as i32, b as i32)
    }

    pub fn a(&self) -> i32 {
        self.0
    }
    pub fn r(&self) -> i32 {
        self.1
    }
    pub fn g(&self) -> i32 {
        self.2
    }
    pub fn b(&self) -> i32 {
        self.3
    }
}

impl VecColor for ARGB {
    fn new_hex(val: u32) -> Self {
        let r = (val & 0x00FF0000) >> 16;
        let g = (val & 0x0000FF00) >> 8;
        let b = val & 0x000000FF;
        let a = val & 0xFF000000;

        ARGB(r as i32, g as i32, b as i32, a as i32)
    }
}

pub trait VecColor {
    fn new_hex(val: u32) -> Self;
}
