use nannou::noise::{NoiseFn, Perlin, Seedable};
use nannou::rand::rngs::StdRng;
use nannou::rand::seq::SliceRandom;
use nannou::rand::{Rng, SeedableRng};
use nannou::{color::rgb_u32, prelude::*};
use paleta_rs;

const ROWS: u32 = 16;
const COLS: u32 = 16;

const SIZE: u32 = 48;
const WIDTH: u32 = COLS * SIZE;
const HEIGHT: u32 = ROWS * SIZE;

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::wait())
        .run();
}

#[derive(Default, Clone)]
struct Point {
    x: f32,
    y: f32,
}

struct Tile {
    x: f32,
    y: f32,
    rotation: f32,
    points: (Point, Point, Point),
    color: Rgb<u8>,
}

struct Model {
    _window: window::Id,
    tiles: Vec<Tile>,
    random_seed: u64,
}

fn get_random_palette(rng: &mut StdRng) -> [u32; 5] {
    let palettes = paleta_rs::all_hex();
    *palettes.choose(rng).unwrap()
}
fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .title(app.exe_name().unwrap())
        .view(view)
        .key_pressed(key_pressed)
        .size(WIDTH, HEIGHT)
        .build()
        .unwrap();

    let mut tiles = Vec::new();
    let points = (
        Point { x: 0.5, y: 0.5 },
        Point { x: -0.5, y: 0.5 },
        Point { x: 0.5, y: -0.5 },
    );
    let random_seed = random_range(0, 100000);

    // let range_x = model.random_range.gen_range(-0.5..0.5);
    // let range_y = model.random_range.gen_range(-1.0..1.0);
    for y in 0..ROWS {
        for x in 0..COLS {
            // let colors = vec![0xCC0C39, 0xE6781E, 0xC8CF02];
            // let color = rgb_u32(colors[random_range::<usize>(0, 3)]);

            let tile = Tile {
                x: x as f32,
                y: y as f32,
                points: points.clone(),
                rotation: 0.0,
                color: BLACK,
            };

            tiles.push(tile);
        }
    }
    Model {
        _window,
        tiles,
        random_seed,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let mut rng = StdRng::seed_from_u64(model.random_seed);
    let palette = get_random_palette(&mut rng);

    for tile in &mut model.tiles {
        let multiplier: i32 = rng.gen_range(0..4);
        let rotation = deg_to_rad(90.0 * multiplier as f32);

        tile.rotation = rotation;
        tile.color = rgb_u32(*palette.choose(&mut rng).unwrap());
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    let gdraw = draw
        .scale(SIZE as f32)
        .scale_y(-1.0)
        .x_y(COLS as f32 / -2.0 + 0.5, ROWS as f32 / -2.0 + 0.5);

    draw.background().color(SNOW);

    for t in model.tiles.iter() {
        let cdraw = gdraw.x_y(t.x, t.y);

        let point1 = pt2(t.points.0.x, t.points.0.y);
        let point2 = pt2(t.points.1.x, t.points.1.y);
        let point3 = pt2(t.points.2.x, t.points.2.y);

        cdraw
            .tri()
            .color(t.color)
            .w_h(1.0, 1.0)
            .rotate(t.rotation)
            .points(point1, point2, point3);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::R => {
            model.random_seed = random_range(0, 1000000);
        }
        Key::S => {
            let screenshot_name =
                format!("{}{:03}.png", app.exe_name().unwrap(), model.random_seed);

            println!("Saving screenshot to: {}", screenshot_name);
            app.main_window().capture_frame(screenshot_name);
        }
        _other_key => {}
    }
}
