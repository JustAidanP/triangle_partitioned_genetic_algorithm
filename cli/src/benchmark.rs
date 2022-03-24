use image_lib::images::grid::GridImage;
use std::time::Instant;

pub fn benchmark() {
    let n = 10;
    let mut foo: i64 = 0;

    let mut image: GridImage<10, 10> = GridImage::new_uniform();
    let start = Instant::now();
    for i in 0..n {
        image.mutate_structure(&image.get_random_inner_vertex(), None);
        image.rasterize_box((8, 8), |point, colour| {
            foo += (point.x + point.y) as i64;
        });
    }
    let now = Instant::now();
    println!(
        "Executing ({}) took {} microseconds; average {} microseconds/image",
        n,
        now.duration_since(start).as_micros(),
        now.duration_since(start).as_micros() / n
    );
    println!("{}", foo);

    let result = 2 + 2;
    assert_eq!(result, 4);
}
