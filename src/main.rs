use nannou::prelude::*;

const ROWS: u32 = 25;
const COLS: u32 = 50;
const SIZE: u32 = 30;
const WIDTH: u32 = COLS * SIZE;
const HEIGHT: u32 = ROWS * SIZE;
const SLANT: f32 = 0.5;

fn main() {
    nannou::sketch(view).size(WIDTH, HEIGHT).loop_mode(LoopMode::loop_once()).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let gdraw = draw.scale(SIZE as f32).scale_y(-1.0).x_y(COLS as f32 / -2.0 + 0.5, ROWS as f32 / -2.0 + 0.5);
    draw.background().color(SNOW);
    let mut counter = 0;
    let mut shade;
    for y in 0..ROWS {
        for x in 0..COLS {
            if counter % 2 == 0 {
                shade = PINK;
            } else {
                shade = BLUE;
            }
            let cdraw = gdraw.x_y(x as f32, y as f32);
            cdraw.polygon()
                .color(shade)
                .points([
                    pt2(0.0, 0.0 + SLANT),
                    pt2(1.0, 0.0 - SLANT),
                    pt2(1.0, 1.0 - SLANT),
                    pt2(0.0, 1.0 + SLANT),
                ]);
            counter += 1;
        }
        counter += 1;
    }
    draw.to_frame(app, &frame).unwrap();
}
