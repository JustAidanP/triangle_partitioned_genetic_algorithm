use super::{Colour, GridImage, GridVertex};
use rand::{random, Rng};
use std::cmp::{max, min};
use std::ops::RangeInclusive;

fn mutate_colour(colour: Colour, mutation_rate: f32) -> Colour {
    if random::<f32>() < mutation_rate {
        let mut thread_rng = rand::thread_rng();

        Colour {
            r: thread_rng.gen_range(
                max(0_i32, colour.r as i32 - 20_i32)..=min(colour.r as i32 + 20_i32, 255_i32),
            ) as u8,
            g: thread_rng.gen_range(
                max(0_i32, colour.g as i32 - 20_i32)..=min(colour.g as i32 + 20_i32, 255_i32),
            ) as u8,
            b: thread_rng.gen_range(
                max(0_i32, colour.b as i32 - 20_i32)..=min(colour.b as i32 + 20_i32, 255_i32),
            ) as u8,
        }
    } else {
        colour
    }
}

impl<const W: usize, const H: usize> GridImage<W, H> {
    /// An implementation of the Vertex-Polygon image mutation
    /// Takes ownership of the image, mutates the underlying memory and returns a new image
    ///
    /// # Arguments
    ///
    /// * `vert` - The specific vertex to mutate in the structure
    /// * `radius` - Defines a circle surrounding the existing point, the new point chosen for
    ///              the vertex must be in this circle, if None is provided, then the surrounding
    ///              circle is uncapped. This is an intersection with the vertex's surrounding
    ///              polygon for valid places to move the vertex to
    ///
    /// # Examples
    /// ```
    /// use image_lib::images::grid::GridImage;
    /// let mut image: GridImage<16, 16> = GridImage::new_uniform();
    /// // Mutates a random inner vertex
    /// image.mutate_structure(&image.get_random_inner_vertex(), None);
    /// ```
    pub fn mutate_structure(&mut self, vert: &GridVertex, radius: Option<u32>) {
        // Picks a random point to move the vertex to
        let point = self.get_random_point_in_vertex_polygon(vert);
        // Mutates the grid image's position function to change the position of the vertex
        self.vertex_positions[vert.vertical][vert.horizontal] = point;
    }

    /// Mutates the colour set of the image
    pub fn mutate_colours(&mut self, mutation_rate: f32) {
        self.colours.map(|row| {
            row.map(|column| {
                // Chooses our base
                let (left, right) = column.clone();

                // Randomly mutates the colour according to the `mutation_rate`
                (
                    mutate_colour(left, mutation_rate),
                    mutate_colour(right, mutation_rate),
                )
            })
        });
    }
}

/// A collection of functions to aid genetic mutation and breeding
impl<const W: usize, const H: usize> GridImage<W, H> {
    pub fn breed(
        left: &GridImage<W, H>,
        right: &GridImage<W, H>,
        mutation_rate: f32,
    ) -> GridImage<W, H> {
        // Randomly chooses to pick the left or right vertex position set
        let vertex_positions = (if random() { left } else { right })
            .get_vertex_positions()
            .clone();

        // Breeds the colours
        let mut x: isize = -1;
        let mut y: isize = -1;
        let colours = [0; H].map(|_| {
            y += 1;
            x = -1;
            [0; W].map(|_| {
                x += 1;
                let left_colour = left.get_colours()[y as usize][x as usize].clone();
                let right_colour = right.get_colours()[y as usize][x as usize].clone();

                // Randomly mutates the colour according to the `mutation_rate`
                (
                    mutate_colour(
                        if random() {
                            left_colour.0
                        } else {
                            right_colour.0
                        },
                        mutation_rate,
                    ),
                    mutate_colour(
                        if random() {
                            left_colour.1
                        } else {
                            right_colour.1
                        },
                        mutation_rate,
                    ),
                )
            })
        });

        let mut image = GridImage {
            vertex_positions,
            colours,
        };

        // There are W * H vertices so we mutate W * H times
        // with chance of mutation in each case being `mutation_rate`
        for _ in 0..(W * H) {
            if random::<f32>() < mutation_rate {
                image.mutate_structure(&image.get_random_inner_vertex(), None);
            }
        }
        image
    }
}
