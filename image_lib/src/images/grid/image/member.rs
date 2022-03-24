use crate::colour::Colour;
use crate::images::grid::image::rasters::Resolution;
use crate::images::grid::GridImage;
use crate::point::Point;
use genetic_algorithm_lib::Member;

pub struct FitnessMetadata {
    /// The offset to perform rasterisation at
    offset: (u16, u16),
    /// The resolution of the fitness calculation, the image is rasterized at this resolution
    resolution: Resolution,
}

impl FitnessMetadata {
    pub fn new(offset: (u16, u16), resolution: Resolution) -> Self {
        FitnessMetadata { offset, resolution }
    }
}

pub struct BreedMetadata {
    /// The mutation rate of the breeding process
    mutation_rate: f32,
}

impl BreedMetadata {
    pub fn new(mutation_rate: f32) -> Self {
        BreedMetadata { mutation_rate }
    }
}

#[derive(Clone)]
pub struct GAImageMember<'a, U, const W: usize, const H: usize>
where
    U: Fn(u16, u16) -> Colour,
{
    /// The image that this GAImageMember encapsulates
    image: GridImage<W, H>,
    /// A function which returns the colour of the target image at the given point, the target
    /// image is assumed to be 65536 x 65536 in dimensions
    get_target_pixel: &'a U,
}

impl<'a, U, const W: usize, const H: usize> GAImageMember<'a, U, W, H>
where
    U: Fn(u16, u16) -> Colour,
{
    pub fn new(image: GridImage<W, H>, get_target_pixel: &'a U) -> Self {
        GAImageMember {
            image,
            get_target_pixel,
        }
    }

    pub fn get_image(&self) -> &GridImage<W, H> {
        &self.image
    }
}

impl<'a, U, const W: usize, const H: usize> Member for GAImageMember<'a, U, W, H>
where
    U: Fn(u16, u16) -> Colour,
{
    type FitnessMetadata = FitnessMetadata;
    type BreedMetadata = BreedMetadata;

    fn fitness(&self, metadata: &Self::FitnessMetadata) -> u64 {
        let Resolution(x_resolution, y_resolution) = metadata.resolution;
        // Calculates the absolute difference between the image and the target
        let mut difference: u64 = 0;
        self.image
            .rasterize_scanline(metadata.resolution, metadata.offset, |p, c| {
                let p = (*self.get_target_pixel)(
                    p.x * x_resolution.get_pixel_size() as u16,
                    p.y * y_resolution.get_pixel_size() as u16,
                );
                let diff = (c.r as i32 - p.r as i32).abs()
                    + (c.g as i32 - p.g as i32).abs()
                    + (c.b as i32 - p.b as i32).abs();
                difference += diff as u64;
            });
        // The fitness is the maximum feasible difference between member and target image - fitness
        3 * 256_u64 * x_resolution.get_pixel_count() as u64 * y_resolution.get_pixel_count() as u64
            - difference
    }

    fn breed(left: &Self, right: &Self, metadata: &Self::BreedMetadata) -> Self {
        GAImageMember {
            image: GridImage::breed(&left.image, &right.image, metadata.mutation_rate),
            get_target_pixel: left.get_target_pixel,
        }
    }
}
