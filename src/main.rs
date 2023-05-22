use nannou::geom::Point2;
use nannou::prelude::*;
use std::time::SystemTime;

const SPEED_LIMIT: f32 = 1f32;
const RECT_COUNT_X: usize = 30;
const RECT_COUNT_Y: usize = 30;
const DECAY_RATE: f32 = 0.02f32;

struct Model {
    _window: window::Id,
    prev_frame_time: SystemTime,
    cur_frame_time: SystemTime,
    time_delta: u128,
    prev_mouse: Point2,
    cur_mouse: Point2,
    mouse_speed: f32,

    grid: Vec<Vec<BonkRect>>,
    focused_rect: (i32, i32),
    color_pos: f32,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let _window: WindowId = app.new_window().view(view).build().unwrap();
    app.window(_window).unwrap().set_cursor_visible(false);
    let now = SystemTime::now();
    let prev_mouse = Point2::new(0f32, 0f32);
    let cur_mouse = prev_mouse.clone();

    Model {
        _window,
        prev_frame_time: now.clone(),
        cur_frame_time: now.clone(),
        time_delta: 0,
        prev_mouse,
        cur_mouse,
        mouse_speed: 0f32,
        grid: vec![vec![BonkRect::new(); RECT_COUNT_Y]; RECT_COUNT_X],
        focused_rect: (0, 0),
        color_pos: 0f32,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // elapsed time
    model.prev_frame_time = model.cur_frame_time;
    model.cur_frame_time = SystemTime::now();
    model.time_delta = model
        .cur_frame_time
        .duration_since(model.prev_frame_time)
        .unwrap()
        .as_millis();

    // elapsed distance
    model.prev_mouse.x = model.cur_mouse.x;
    model.prev_mouse.y = model.cur_mouse.y;

    //calculate the mouse speed
    model.cur_mouse.x = _app.mouse.x;
    model.cur_mouse.y = _app.mouse.y;
    let (x1, y1, x2, y2) = (
        model.prev_mouse.x,
        model.prev_mouse.y,
        model.cur_mouse.x,
        model.cur_mouse.y,
    );
    let distance = ((x2 - x1).powf(2.0) + (y2 - y1).powf(2.0)).sqrt();
    model.mouse_speed = distance / model.time_delta as f32;

    // make sure the rects are drawn appropriately
    update_rects(_app, model);

    // rotate color based on speed
    if !f32::is_nan(model.mouse_speed) {
        model.color_pos += model.mouse_speed * 0.5f32;
    }

    if model.color_pos > 360f32 {
        model.color_pos -= 360f32
    }

    // did we bonk a new rect?
    let cur_bonked_rect = get_rect_mouse_is_in(_app);
    let did_bonk_new_rect =
        model.focused_rect.0 != cur_bonked_rect.0 || model.focused_rect.1 != cur_bonked_rect.1;
    model.focused_rect = cur_bonked_rect;
    let (x, y) = model.focused_rect;

    let speed_percent = f32::min(model.mouse_speed / SPEED_LIMIT, SPEED_LIMIT) / SPEED_LIMIT;

    if did_bonk_new_rect {
        let bonk_rect = &mut model.grid[x as usize][y as usize];
        bonk_rect.excited = speed_percent;
        bonk_rect.hue = model.color_pos;

        bonk_rect.sleeping = false;
    }
}

#[derive(Debug, Clone)]
struct BonkRect {
    point: Point2,
    dims: Vec2,
    excited: f32,
    hue: f32,
    sleeping: bool,
}

impl BonkRect {
    fn new() -> BonkRect {
        BonkRect {
            point: Point2::new(0f32, 0f32),
            dims: Vec2::new(1f32, 1f32),
            excited: 0f32,
            hue: 0f32,
            sleeping: true,
        }
    }
}

fn update_rects(app: &App, model: &mut Model) {
    let (win_width, win_height) = app.main_window().inner_size_points();
    let (rect_width, rect_height) = (
        win_width / RECT_COUNT_X as f32,
        win_height / RECT_COUNT_Y as f32,
    );
    // i think its a xy grid with 0,0 in the center, and i want it the bottom left... but its not totally exact
    // hence the .2 part.. dunno what the fuck thats all about
    let x_offset = win_width / 2.2f32;
    let y_offset = win_height / 2.2f32;

    for x in 0..RECT_COUNT_X {
        for y in 0..RECT_COUNT_Y {
            let rect = &mut model.grid[x][y];
            rect.point.x = x as f32 * rect_width - x_offset;
            rect.point.y = y as f32 * rect_height - y_offset;
            // the minus 5 gives a space
            rect.dims.x = rect_width - 1f32;
            rect.dims.y = rect_height - 1f32;

            if rect.excited < 0.001f32 && !rect.sleeping {
                rect.sleeping = true;
            }

            if rect.excited > 0f32 {
                // slow decay
                rect.excited -= rect.excited * DECAY_RATE;
            } else {
                rect.excited = 0f32;
            }
        }
    }
}

fn get_rect_mouse_is_in(app: &App) -> (i32, i32) {
    let (win_width, win_height) = app.main_window().inner_size_points();
    // again, trying to treat the grid as a 0,0 being in the bottom left, but not totally exact
    // trying to map cursor pos to rects
    let (x_offset, y_offset) = (win_width / 2.05f32, win_height / 2.05f32);

    let x_rect_idx = ((app.mouse.x + x_offset) * RECT_COUNT_X as f32) as i32 / win_width as i32;
    let y_rect_idx = ((app.mouse.y + y_offset) * RECT_COUNT_Y as f32) as i32 / win_height as i32;

    (x_rect_idx, y_rect_idx)
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    for x in 0..RECT_COUNT_X {
        for y in 0..RECT_COUNT_Y {
            if !model.grid[x][y].sleeping {
                let BonkRect {
                    point,
                    dims,
                    excited,
                    hue,
                    ..
                } = model.grid[x][y];
                draw.rect()
                    .xy(point)
                    .wh(dims)
                    .color(Hsl::new(hue, 1f32, 0.5f32 * excited));
            }
        }
    }

    // cursor
    // draw.ellipse()
    //     .xy(model.cur_mouse)
    //     .radius(10f32)
    //     .color(cl);

    draw.to_frame(app, &frame).unwrap();
}
