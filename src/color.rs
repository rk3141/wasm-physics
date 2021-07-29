use std::fmt::format;

#[derive(Copy, Clone)]
pub struct Color(u8, u8, u8);

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { 0: r, 1: b, 2: g }
    }

    pub fn get_hex(&self) -> String {
        let (r, g, b) = (self.0, self.1, self.2);

        let mut rstr = format!("{:x}", r);
        let mut gstr = format!("{:x}", g);
        let mut bstr = format!("{:x}", b);

        if r < 16 {
            rstr = format!("0{:x}", r);
        }

        if g < 16 {
            gstr = format!("0{:x}", g);
        }

        if b < 16 {
            bstr = format!("0{:x}", b);
        }

        format!("{}{}{}", rstr, gstr, bstr)
    }
}
