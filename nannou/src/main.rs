use image_lib::images::grid::GridImage;
use nannou::prelude::*;
use nannou::rand::rand;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::mem::swap;
use std::rc::Rc;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    image: GridImage<8, 8>,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    let image: GridImage<8, 8> = GridImage::new_uniform();

    Model { _window, image }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    _model.image.borrow_mut().mutate_structure();
    _model.image.borrow_mut().mutate_structure();
    _model.image.borrow_mut().mutate_structure();
    _model.image.borrow_mut().mutate_structure();
    _model.image.borrow_mut().mutate_structure();
    _model.image.borrow_mut().mutate_structure();
    _model.image.borrow_mut().mutate_structure();
    _model.image.borrow_mut().mutate_structure();
    _model.image.borrow_mut().mutate_colour();
    _model.image.borrow_mut().mutate_colour();
    _model.image.borrow_mut().mutate_colour();
    _model.image.borrow_mut().mutate_colour();
    _model.image.borrow_mut().mutate_colour();
    _model.image.borrow_mut().mutate_colour();
    _model.image.borrow_mut().mutate_colour();
    _model.image.borrow_mut().mutate_colour();
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    // draw.ellipse().color(STEELBLUE);
    println!("FPS {}", app.fps());

    // let v1 = (0, 10);
    // let v2 = (0, 2);
    // let v3 = (rand::random::<i32>() % 15, 0);
    // println!("v3.x: {}", v3.0);
    // for y in 0..10 {
    //     // Gets v1 -> v3 (as v1 is highest and v3 is lowest) x
    //     let long_x = v1.0 + ((v3.0 - v1.0) * (y - v1.1)) / (v3.1 - v1.1);
    //     println!("x: {} at y: {}", long_x, y);
    //
    //     // Decides which short side we are getting x for
    //     let mut short_x = 0;
    //     if y > v2.1 {
    //         short_x = ((y - v1.1) * (v2.0 - v1.0)) / (v2.1 - v1.1);
    //     } else {
    //         short_x = ((y - v2.1) * (v3.0 - v2.0)) / (v3.1 - v2.1);
    //     }
    //     for x in if short_x < long_x {
    //         short_x..long_x
    //     } else {
    //         long_x..short_x
    //     } {
    //         draw.rect()
    //             .color(STEELBLUE)
    //             .x(x as f32 * 10 as f32)
    //             .y(y as f32 * 10 as f32)
    //             .width(10.0)
    //             .height(10.0);
    //     }
    // }

    let x = 7;
    let mut draws = 0;
    /// Rasterizes to a resolution of (x, x), i.e. every 2^x pixels rasterizes to a single pixel and there are 65536 / 2^x pixels in each axis
    _model.image.rasterize_scanline((x, x), |point, colour| {
        draws += 1;
        // Draws the pixel, centering the middle of the image
        draw.rect()
            .rgb8(colour.r, colour.g, colour.b)
            .x(point.x as f32 - (65536 / ((2 as u32).pow(x as u32)) / 2) as f32)
            .y(point.y as f32 - (65536 / ((2 as u32).pow(x as u32)) / 2) as f32)
            .width(1.0)
            .height(1.0);
    });
    println!("Draws: {}", draws);

    // _model.image.get_triangles(|first, second, third, colour| {
    //     draw.polygon()
    //         .rgb8(colour.r, colour.g, colour.b)
    //         .points([
    //             pt2(first.x as f32 - 256.0, first.y as f32 - 256.0),
    //             pt2(second.x as f32 - 256.0, second.y as f32 - 256.0),
    //             pt2(third.x as f32 - 256.0, third.y as f32 - 256.0)
    //         ]);
    // });

    draw.to_frame(app, &frame).unwrap();
}
