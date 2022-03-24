extern crate core;

pub mod colour;
pub mod images;
pub mod point;

/**

 Debug Mode (Non Release)

    - Mutate + Rasterize[Empty loops] -> n: 2,500,000 total: 37325060 microseconds avg: 14 microseconds/image
    - Mutate + Rasterize[Init tri] -> n: 2,500,000 total: 46661794 microseconds avg: 18 microseconds/image
    - Mutate + Rasterize[Init edges] -> n: 2,500,000 total: 82660394 microseconds avg: 33 microseconds/image
    - Mutate + Rasterize[Edge Draw] -> n: 2,500,000 total: 112459741 microseconds avg: 44 microseconds/image
    - Mutate + Rasterize[Inefficient Box Raster] -> n: 250,000 total: 194210792 microseconds avg: 776 microseconds/image

*/

#[cfg(test)]
mod tests {
    use crate::images::grid::GridImage;
    use std::time::Instant;

    #[test]
    fn it_works() {
        let n: u64 = 25000;
        let mut foo: i64 = 0;

        let mut image: GridImage<16, 16> = GridImage::new_uniform();

        let mut points_rendered: u64 = 0;

        let start = Instant::now();
        for i in 0..n {
            // image.mutate_structure();
            image.rasterize_scanline((8, 8), (0, 0), |point, colour| {
                foo += (point.x + point.y) as i64;
                points_rendered += 1;
            });
        }
        let now = Instant::now();
        println!(
            "Executing ({}) took {} microseconds; average {} microseconds/image",
            n,
            now.duration_since(start).as_micros(),
            now.duration_since(start).as_micros() / (n as u128)
        );
        println!("{}", foo);
        println!(
            "{} points were rendered, averaging {} points/image",
            points_rendered,
            points_rendered / n
        );

        let result = 2 + 2;
        assert_eq!(result, 1);
    }
}
