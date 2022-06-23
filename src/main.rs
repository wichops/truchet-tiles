use nannou::prelude::*;

const ROWS: u32 = 24;
const COLS: u32 = 24;

const SIZE: u32 = 20;

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
}

struct Model {
    _window: window::Id,
    tiles: Vec<Tile>,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .view(view)
        .key_pressed(key_pressed)
        .size(WIDTH, HEIGHT)
        .build()
        .unwrap();

    let mut tiles = Vec::new();

    for y in 0..ROWS {
        for x in 0..COLS {
            let points = (
                Point { x: 0.5, y: 0.5 },
                Point { x: -0.5, y: 0.5 },
                Point { x: 0.5, y: -0.5 },
            );

            let rotation: f32 = match random_range::<i32>(0, 4) {
                0 => deg_to_rad(90.0),
                1 => deg_to_rad(180.0),
                2 => deg_to_rad(270.0),
                _ => deg_to_rad(0.0),
            };
            let tile = Tile {
                x: x as f32,
                y: y as f32,
                points,
                rotation,
            };

            tiles.push(tile);
        }
    }
    Model { _window, tiles }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

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
            .color(BLACK)
            .w_h(1.0, 1.0)
            .rotate(t.rotation)
            .points(point1, point2, point3);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn key_pressed(app: &App, _model: &mut Model, key: Key) {
    match key {
        Key::S => {
            app.main_window()
                .capture_frame(app.exe_name().unwrap() + ".png");
        }
        _other_key => {}
    }
}
