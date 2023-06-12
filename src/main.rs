use nannou::prelude::*;


const ROWS: u32 = 25;
const COLS: u32 = 50;
const SIZE: u32 = 30;
const WIDTH: u32 = COLS * SIZE;
const HEIGHT: u32 = ROWS * SIZE;

fn main() {
    nannou::sketch(view).size(WIDTH, HEIGHT).loop_mode(LoopMode::loop_once()).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let gdraw = draw.scale(SIZE as f32).scale_y(-1.0).x_y(COLS as f32 / -2.0 + 0.5, ROWS as f32 / -2.0 + 0.5);
    draw.background().color(SNOW);
    let mut counter = 0;
    let mut shade = PINK;
    for y in 0..ROWS {
        for x in 0..COLS {
            if counter % 2 == 0 {
                shade = PINK;
            } else {
                shade = RED;
            }
            let cdraw = gdraw.x_y(x as f32, y as f32);
            cdraw.rect()
                .color(shade)
                .w_h(1.0, 1.0);
            counter += 1;
        }
        counter += 1;
    }
    draw.to_frame(app, &frame).unwrap();
}
