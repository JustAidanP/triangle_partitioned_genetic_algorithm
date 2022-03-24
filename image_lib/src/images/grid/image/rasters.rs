use super::{Colour, GridImage, GridVertex, Point, Tri};
use std::cmp::{max, min};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AxisResolution {
    Blocks1,
    Blocks2,
    Blocks4,
    Blocks8,
    Blocks16,
    Blocks32,
    Blocks64,
    Blocks128,
    Blocks256,
    Blocks512,
    Blocks1024,
    Blocks2048,
    Blocks4096,
    Blocks8192,
    Blocks16384,
    Blocks32768,
    Blocks65536,
}

impl AxisResolution {
    pub fn get_pixel_size(self: &Self) -> u32 {
        match self {
            AxisResolution::Blocks1 => 65536,
            AxisResolution::Blocks2 => 32768,
            AxisResolution::Blocks4 => 16384,
            AxisResolution::Blocks8 => 8192,
            AxisResolution::Blocks16 => 4096,
            AxisResolution::Blocks32 => 2048,
            AxisResolution::Blocks64 => 1024,
            AxisResolution::Blocks128 => 512,
            AxisResolution::Blocks256 => 256,
            AxisResolution::Blocks512 => 128,
            AxisResolution::Blocks1024 => 64,
            AxisResolution::Blocks2048 => 32,
            AxisResolution::Blocks4096 => 16,
            AxisResolution::Blocks8192 => 8,
            AxisResolution::Blocks16384 => 4,
            AxisResolution::Blocks32768 => 2,
            AxisResolution::Blocks65536 => 1,
        }
    }
    pub fn get_pixel_count(self: &Self) -> u32 {
        match self {
            AxisResolution::Blocks1 => 1,
            AxisResolution::Blocks2 => 2,
            AxisResolution::Blocks4 => 4,
            AxisResolution::Blocks8 => 8,
            AxisResolution::Blocks16 => 16,
            AxisResolution::Blocks32 => 32,
            AxisResolution::Blocks64 => 64,
            AxisResolution::Blocks128 => 128,
            AxisResolution::Blocks256 => 256,
            AxisResolution::Blocks512 => 512,
            AxisResolution::Blocks1024 => 1024,
            AxisResolution::Blocks2048 => 2048,
            AxisResolution::Blocks4096 => 4096,
            AxisResolution::Blocks8192 => 8192,
            AxisResolution::Blocks16384 => 16384,
            AxisResolution::Blocks32768 => 32768,
            AxisResolution::Blocks65536 => 65536,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Resolution(pub AxisResolution, pub AxisResolution);

impl Resolution {
    /// Gets the size of the pixel blocks in each axis
    pub fn get_pixel_size(self: Self) -> (u32, u32) {
        (self.0.get_pixel_size(), self.1.get_pixel_size())
    }

    /// Gets the number of pixels in each axis
    pub fn get_pixel_count(self: Self) -> (u32, u32) {
        (self.0.get_pixel_count(), self.1.get_pixel_count())
    }
}

impl<const W: usize, const H: usize> GridImage<W, H> {
    /// Internally all points are defined to be within `0..u16::MAX`.
    /// those ranges respectively.
    pub fn get_triangles<F>(&self, mut on_triangle: F)
    where
        F: FnMut(Point, Point, Point, Colour),
    {
        // For each vertex (excluding right and bottom edge), return its two triangles
        for horizontal in 0..(W - 1) {
            for vertical in 0..(H - 1) {
                let vert = GridVertex {
                    horizontal,
                    vertical,
                };
                on_triangle(
                    self.get_vert_position(&vert),
                    self.get_vert_position(&vert.right()),
                    self.get_vert_position(&vert.down_right()),
                    self.get_vert_colours(&vert).0,
                );
                on_triangle(
                    self.get_vert_position(&vert),
                    self.get_vert_position(&vert.down_right()),
                    self.get_vert_position(&vert.down()),
                    self.get_vert_colours(&vert).1,
                );
            }
        }
    }

    /// Internally all points are defined to be within `0..u16::MAX`.
    /// The `resolution` parameter is a tuple defining the size of the grid, the value can be
    /// between 0 and 16, the number of pixels in the grid (in each axis) is determined by
    /// 65536 / 2^x, it  also follows that the size of each pixel is 2^x.
    /// e.g. if the resolution is (4, 4), then every square of 16 x 16 pixels gets rasterized
    /// as a single pixel and there are in total 4096 x 4096 pixels in the grid. Or as a reasonable
    /// size, take a resolution of (8, 8), there are then 256 x 256 pixels in the grid and
    /// each square of 256 x 256 pixels gets rasterized as a single pixel
    pub fn rasterize_box<F>(&self, resolution: (u8, u8), mut on_point: F)
    where
        F: FnMut(Point, Colour),
    {
        assert!(
            0 <= resolution.0 && resolution.0 <= 16,
            "The resolution must be between 0 and 16 (inclusive)"
        );
        assert!(
            0 <= resolution.1 && resolution.1 <= 16,
            "The resolution must be between 0 and 16 (inclusive)"
        );
        // Calculates the point scalar, i.e. calculates the size of each rasterized pixel
        let scale = (
            (2 as u16).pow(resolution.0 as u32),
            (2 as u16).pow(resolution.1 as u32),
        );
        // For each vertex (excluding right and bottom edge), gets its tile
        for horizontal in 0..(W - 1) {
            for vertical in 0..(H - 1) {
                let vert = GridVertex {
                    horizontal,
                    vertical,
                };
                // Triangle including top and right edge of tile
                {
                    let vert_pos = self.get_vert_position(&vert).scale_down(scale);
                    let right_pos = self.get_vert_position(&vert.right()).scale_down(scale);
                    let right_down_pos =
                        self.get_vert_position(&vert.down_right()).scale_down(scale);

                    // Only renders the top and right edge of tile
                    Tri::new(vert_pos, right_pos, right_down_pos).rasterize(
                        true,
                        true,
                        false,
                        |p| {
                            on_point(p, self.get_vert_colours(&vert).0);
                        },
                    );
                }
                // Triangle including left and bottom edge of tile
                {
                    let vert_pos = self.get_vert_position(&vert).scale_down(scale);
                    let right_down_pos =
                        self.get_vert_position(&vert.down_right()).scale_down(scale);
                    let down_pos = self.get_vert_position(&vert.down()).scale_down(scale);

                    let tri = Tri::new(vert_pos, right_down_pos, down_pos);
                    // Render diag of tile
                    let render_diag = true;
                    // If left most edge, render left edge of tile
                    let render_left = false;
                    // if bottom most edge, render bottom edge of tile
                    let render_bottom = false;
                    tri.rasterize(render_diag, render_bottom, render_left, |p| {
                        on_point(p, self.get_vert_colours(&vert).1);
                    });
                }
            }
        }
    }

    /// Internally all points are defined to be within `0..u16::MAX`.
    ///
    /// * `resolution` - The number of pixels that constitute a raster pixel in each axis, this
    ///                  will then call on_point with raster pixels. e.g. With a resolution of
    ///                  AxisResolution::Blocks64 (64 raster pixels), the i'th (0..64) raster pixel
    ///                  is the (i*1024)'s pixel's colour in the image  
    ///                  
    pub fn rasterize_scanline<F>(&self, resolution: Resolution, offset: (u16, u16), mut on_point: F)
    where
        F: FnMut(Point, Colour),
    {
        let Resolution(x_resolution, y_resolution) = resolution;
        // The number of domain pixels that constitute a raster pixel
        let step = (
            x_resolution.get_pixel_size() as i64,
            y_resolution.get_pixel_size() as i64,
        );
        // Loops through every triangle and works out how many points raster points lie in the
        // boundary; if a raster pixel constitutes 16 x 16 pixels, then the raster pixel is (0, 0)
        self.get_triangles(|first, second, third, colour| {
            // First sorts the verticies by height
            let mut sorted = [first, second, third];
            sorted.sort_by(|l, r| r.y.cmp(&l.y));
            let v1 = sorted[0];
            let v2 = sorted[1];
            let v3 = sorted[2];

            // If v1.y = v3.y then we know that the area of the tri must be 0
            if v1.y == v3.y {
                return;
            } else {
                let v1 = (v1.x as i64 - offset.0 as i64, v1.y as i64 - offset.1 as i64);
                let v2 = (v2.x as i64 - offset.0 as i64, v2.y as i64 - offset.1 as i64);
                let v3 = (v3.x as i64 - offset.0 as i64, v3.y as i64 - offset.1 as i64);

                // The start value of y, this is closest multiple of step.1 s.t. start >= v3.y
                let start_y = if v3.1 % step.1 == 0 {
                    v3.1
                } else {
                    step.1 * (v3.1 / step.1 + 1)
                };
                // println!("start_y: {}, end_y: {}", start_y, v1.1);
                for y in (start_y..v1.1).step_by(step.1 as usize) {
                    // Gets v1 -> v3 (as v1 is highest and v3 is lowest) x
                    let long_x = v1.0 + ((v3.0 - v1.0) * (y - v1.1)) / (v3.1 - v1.1);

                    // If v1.x == v2.x then it is assumed that the tri containing v1 & v2 has zero area
                    if v1.1 == v2.1 && v2.1 == y {
                    } else if v2.1 == v3.1 && v2.1 == y { // Likewise for the v2 & v3 tri
                    } else {
                        // Decides which short side we are getting x for
                        let mut short_x = 0;
                        if y > v2.1 {
                            short_x = v1.0 + ((y - v1.1) * (v2.0 - v1.0)) / (v2.1 - v1.1);
                        } else {
                            short_x = v2.0 + ((y - v2.1) * (v3.0 - v2.0)) / (v3.1 - v2.1);
                        }

                        let start_x = if short_x < long_x { short_x } else { long_x };
                        let end_x = if short_x < long_x { long_x } else { short_x };

                        // The start value of x, this is closest multiple of step.0 s.t. start >= start_x
                        let start_x = if start_x % step.0 == 0 {
                            start_x
                        } else {
                            step.0 * (start_x / step.0 + 1)
                        };
                        // println!("start_x: {}, end_x: {}", start_x, end_x);
                        for x in (start_x..end_x).step_by(step.0 as usize) {
                            on_point(
                                // Scales the point to a 'raster pixel'
                                Point {
                                    x: (x / x_resolution.get_pixel_size() as i64) as u16,
                                    y: (y / x_resolution.get_pixel_size() as i64) as u16,
                                },
                                colour,
                            );
                        }
                    }
                }
            }
        });
    }
}
