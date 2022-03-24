#![feature(int_abs_diff)]

use genetic_algorithm_lib::Member;
use image::imageops::{resize, FilterType};
use image::{ImageBuffer, RgbImage};
use image_lib::colour::Colour;
use image_lib::images::grid::{AxisResolution, GridImage, Resolution};

#[derive(Clone)]
pub struct ImageMember<'a, const W: usize, const H: usize> {
    image: GridImage<W, H>,
    target: &'a RgbImage,
}

impl<'a, const W: usize, const H: usize> ImageMember<'a, W, H> {
    pub fn new(target: &'a RgbImage) -> ImageMember<'a, W, H> {
        ImageMember {
            image: GridImage::new_uniform(),
            target,
        }
    }

    pub fn get_image(&self) -> &GridImage<W, H> {
        &self.image
    }
}

impl<'a, const W: usize, const H: usize> Member for ImageMember<'a, W, H> {
    type FitnessMetadata = ();
    type BreedMetadata = ();

    fn fitness(&self, metadata: &Self::FitnessMetadata) -> u64 {
        // The raster resolution, the total number of raster pixels (in each axis) is 65536 / (2^x)
        let resolution = (8, 8);
        // Scale to map a raster pixel to the target
        let raster_to_target_x = |raster_x: u32| {
            raster_x * self.target.width() / (2 as u32).pow(16 - resolution.0 as u32)
        };
        let raster_to_target_y = |raster_y: u32| {
            raster_y * self.target.height() / (2 as u32).pow(16 - resolution.1 as u32)
        };

        let mut difference: u64 = 0;
        let mut rastered = 0;
        let offset: (u16, u16) = (
            0, 0
            // rand::random::<u16>() % 2_u16.pow(resolution.0 as u32),
            // rand::random::<u16>() % 2_u16.pow(resolution.1 as u32),
        );
        self.image.rasterize_scanline(
            Resolution(AxisResolution::Blocks64, AxisResolution::Blocks64),
            offset,
            |p, c| {
                rastered += 1;
                let p = *self.target.get_pixel(
                    raster_to_target_x(p.x as u32),
                    raster_to_target_y(p.y as u32),
                );
                let diff = (c.r as i32 - p[0] as i32).abs()
                    + (c.g as i32 - p[1] as i32).abs()
                    + (c.b as i32 - p[2] as i32).abs();
                difference += diff as u64;
            },
        );
        // The fitness is the maximum feasible difference between member and target image - fitness
        3 * u8::MAX as u64
            * (u16::MAX as u64 / 2_u64.pow(resolution.0 as u32))
            * (u16::MAX as u64 / 2_u64.pow(resolution.1 as u32))
            - difference
    }

    /// Breeds together two images, if the targets differ (they **shouldn't**), the left is chosen
    fn breed(left: &Self, right: &Self, metadata: &Self::BreedMetadata) -> Self {
        ImageMember {
            image: GridImage::breed(&left.image, &right.image, 0.05),
            target: left.target,
        }
    }
}
