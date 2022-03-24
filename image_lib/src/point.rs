#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Point {
        Point { x, y }
    }

    // Scales up the x and y coordinates
    pub fn scale_up(&self, scale: (u16, u16)) -> Point {
        Point {
            x: self.x * scale.0,
            y: self.y * scale.1,
        }
    }

    // Scales down the x and y coordinates, e.g. a scale of 2 would half the values
    pub fn scale_down(&self, scale: (u16, u16)) -> Point {
        Point {
            x: self.x / scale.0,
            y: self.y / scale.1,
        }
    }
}
