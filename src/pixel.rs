pub trait Pixel: 'static + Copy + Eq {
    fn white() -> Self;
    fn black() -> Self;
    fn to_rgba(&self) -> Rgba;
    fn to_rgb(&self) -> Rgb {
        self.to_rgba().to_rgb()
    }
    fn to_grey(&self) -> u8 {
        self.to_rgb().to_grey()
    }
    fn to_bit(&self) -> bool {
        self.to_grey().to_bit()
    }
    fn r(&self) -> u8 {
        self.to_rgba().r
    }
    fn g(&self) -> u8 {
        self.to_rgba().g
    }
    fn b(&self) -> u8 {
        self.to_rgba().b
    }
    fn a(&self) -> u8 {
        self.to_rgba().a
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Pixel for Rgba {
    fn white() -> Self {
        Self {
            r: 0xff,
            g: 0xff,
            b: 0xff,
            a: 0xff,
        }
    }
    fn black() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 0xff,
        }
    }
    fn to_rgba(&self) -> Rgba {
        *self
    }
    fn to_rgb(&self) -> Rgb {
        let Rgba { r, g, b, a: _ } = *self;
        Rgb { r, g, b }
    }
    fn to_grey(&self) -> u8 {
        self.to_rgb().to_grey()
    }
    fn to_bit(&self) -> bool {
        self.to_grey().to_bit()
    }
}

impl Pixel for Rgb {
    fn white() -> Self {
        Self {
            r: 0xff,
            g: 0xff,
            b: 0xff,
        }
    }
    fn black() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }
    fn to_rgba(&self) -> Rgba {
        let Rgb { r, g, b } = *self;
        Rgba { r, g, b, a: 0xff }
    }
    fn to_rgb(&self) -> Rgb {
        *self
    }
    fn to_grey(&self) -> u8 {
        let Rgb { r, g, b } = *self;
        (r as f64 * 0.299 + g as f64 * 0.587 + b as f64 * 0.114).round() as u8
    }
    fn to_bit(&self) -> bool {
        self.to_grey().to_bit()
    }
}

impl Pixel for u8 {
    fn white() -> Self {
        0xff
    }
    fn black() -> Self {
        0
    }
    fn to_rgba(&self) -> Rgba {
        self.to_rgb().to_rgba()
    }
    fn to_rgb(&self) -> Rgb {
        Rgb {
            r: *self,
            g: *self,
            b: *self,
        }
    }
    fn to_grey(&self) -> u8 {
        *self
    }
    fn to_bit(&self) -> bool {
        *self == 0
    }
}

impl Pixel for bool {
    fn white() -> Self {
        false
    }
    fn black() -> Self {
        true
    }
    fn to_rgba(&self) -> Rgba {
        self.to_rgb().to_rgba()
    }
    fn to_rgb(&self) -> Rgb {
        self.to_grey().to_rgb()
    }
    fn to_grey(&self) -> u8 {
        if *self { 0 } else { 0xff }
    }
    fn to_bit(&self) -> bool {
        *self
    }
}
