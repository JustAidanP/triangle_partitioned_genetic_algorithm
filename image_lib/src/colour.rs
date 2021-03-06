#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Colour {
    /// Creates a new colour with corresponding parameters
    pub fn new(r: u8, g: u8, b: u8) -> Colour {
        Colour { r, g, b }
    }
}
