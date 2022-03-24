mod benchmark;
mod image_member;

use chrono::{Datelike, Timelike, Utc};
use std::fs::File;
use std::io::Write;
use std::time::Instant;

use genetic_algorithm_lib::{Member, Population};
use image::{io::Reader, open, DynamicImage, Rgb, RgbImage};
use imageproc::{drawing::draw_polygon, point::Point};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

use cairo;
use cairo::{Context, Format, ImageSurface, SvgSurface};
use image::imageops::{resize, FilterType};
use image_lib::colour::Colour;
use image_lib::images::grid::{
    AxisResolution, BreedMetadata, FitnessMetadata, GridImage, Resolution,
};
use rand::Rng;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Target Image Mode",
    about = "Genetic Evolution towards a target image"
)]
struct TargetImage {
    /// The image that the genetic evolution should target
    #[structopt(parse(from_os_str))]
    target: PathBuf,
    /// The number of generations to perform,
    /// if not provided, will run indefinitely or until any target time has elapsed
    #[structopt(short, long)]
    generations: Option<usize>,
    /// How long to perform genetic evolution for in seconds,
    /// if not provided, will run indefinitely or until any target
    /// generation count has been reached
    #[structopt(short, long)]
    time: Option<usize>,
}

fn do_genetic_evolution(args: TargetImage) {
    // Loads the image
    let target = Reader::open(args.target)
        .unwrap()
        .decode()
        .unwrap()
        .to_rgb8();
    let (target_width, target_height) = (target.width(), target.height());
    println!("Loaded target file");

    // A closure to get the colour of a pixel at a point in the target, it assumes that x and y is
    // already normalised to between 0..65536
    let get_target_pixel = |x: u16, y: u16| {
        let colour = target.get_pixel(
            x as u32 * target_width / 65536_u32,
            y as u32 * target_height / 65536_u32,
        );
        Colour::new(colour[0], colour[1], colour[2])
    };
    // Creates the initial population
    let mut population = Box::new(Population::new([0; 25].map(|_| {
        image_lib::images::grid::GAImageMember::new(
            GridImage::<16, 16>::new_uniform(),
            &get_target_pixel,
        )
    })));

    // Goes into main loop
    let start_time = Utc::now();
    let mut generation = 0;
    while match args.generations {
        Some(n) => generation < n,
        None => true,
    } && match args.time {
        Some(n) => (Utc::now().time() - start_time.time()).num_seconds() < n as i64,
        None => true,
    } {
        // Runs the population
        let resolution = Resolution(AxisResolution::Blocks64, AxisResolution::Blocks64);
        let fitness_metadata = FitnessMetadata::new(
            (
                rand::thread_rng().gen_range(0..(resolution.0.get_pixel_size())) as u16,
                rand::thread_rng().gen_range(0..(resolution.1.get_pixel_size())) as u16,
            ),
            resolution,
        );
        let executed_population = population.run(&fitness_metadata);
        let best = executed_population.best().0;

        if generation % 250 == 0 {
            let now = Utc::now();
            println!(
                "Best fitness for generation {} is {} with worst being {} with time elapsed being {}h, {}m, {}s",
                generation,
                executed_population.best().1,
                executed_population.worst().1,
                (now.time() - start_time.time()).num_hours(),
                (now.time() - start_time.time()).num_minutes(),
                (now.time() - start_time.time()).num_seconds()
            );

            // Exports the best members, image
            let mut export = ImageSurface::create(Format::Rgb24, 3 * 1024, 1024).unwrap();
            let context = Context::new(&export).unwrap();

            // Renders the target image
            let resolution = Resolution(AxisResolution::Blocks1024, AxisResolution::Blocks1024);
            let Resolution(x_resolution, y_resolution) = resolution;

            best.get_image()
                .rasterize_scanline(resolution, (0, 0), |p, c| {
                    let tp = get_target_pixel(
                        (p.x as u32 * x_resolution.get_pixel_size()) as u16,
                        (p.y as u32 * y_resolution.get_pixel_size()) as u16,
                    );
                    // Renders the difference
                    context.set_source_rgb(
                        (c.r as i32 - tp.r as i32).abs() as f64 / 256.0,
                        (c.g as i32 - tp.g as i32).abs() as f64 / 256.0,
                        (c.b as i32 - tp.b as i32).abs() as f64 / 256.0,
                    );
                    context.rectangle(p.x as f64, p.y as f64, 1.0, 1.0);
                    context.fill();

                    // Renders the target image pixel
                    context.set_source_rgb(
                        tp.r as f64 / 256 as f64,
                        tp.g as f64 / 256 as f64,
                        tp.b as f64 / 256 as f64,
                    );
                    context.rectangle(p.x as f64 * 1.0 + 1024.0, p.y as f64 * 1.0, 1.0, 1.0);
                    context.fill();

                    // Renders the image pixel
                    context.set_source_rgb(
                        c.r as f64 / 256 as f64,
                        c.g as f64 / 256 as f64,
                        c.b as f64 / 256 as f64,
                    );
                    context.rectangle(p.x as f64 * 1.0 + 2048.0, p.y as f64 * 1.0, 1.0, 1.0);
                    context.fill();
                });

            // export.finish();
            let mut f = std::fs::File::create(Path::new(
                format!(
                    "./foo/gen_{}_fitness_{}.png",
                    generation,
                    executed_population.best().1
                )
                .as_str(),
            ))
            .unwrap();
            export.write_to_png(&mut f);
        }
        generation += 1;
        // Applies natural selection to get the next generation
        let breed_metadata = BreedMetadata::new(0.05);
        population = Box::new(executed_population.breed(&breed_metadata));
    }

    // Prints final results
    println!(
        "Genetic Evolution started at {} and finished at {}, performing {} generations",
        start_time.to_rfc3339(),
        Utc::now().to_rfc3339(),
        generation
    );
}

fn main() {
    // Target Image Mode -> Genetic Evolution towards a target image
    let args = TargetImage::from_args();
    println!("{:?}", args);
    do_genetic_evolution(args);
}
