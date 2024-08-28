use std::{f64::consts::PI, time::Instant};

use subsys::create_subsys;

mod input;
mod subsys;

use input::Input;
use sdl2::{pixels::Color, rect::Point};

fn main() {
    // println!("program start");
    const WIN_WIDTH: u32 = 800;
    const WIN_HEIGHT: u32 = 600;
    const WIDTH: u32 = 80;
    const HEIGHT: u32 = 60;

    const COLORSPEED: f64 = 500.0;

    let (sdl_context, mut canvas, _texture_creator) =
        create_subsys(WIN_WIDTH, WIN_HEIGHT, WIDTH, HEIGHT);

    let mut input = Input::build(sdl_context.event_pump().unwrap());

    let bg_color = Color::RGB(50, 50, 50);
    let fg_color = Color::RGB(255, 255, 255);
    let mut neo_color = fg_color;

    let mut time: Instant;
    let mut framecount: u64 = 0;
    // println!("init arrays");

    // #[allow(unused_assignments)]
    // let mut old_data = [[bg_color; WIDTH as usize]; HEIGHT as usize];
    // let mut data = [[bg_color; WIDTH as usize]; HEIGHT as usize];
    let mut vec_old_rows;
    let mut vec_rows: Vec<[Color; WIDTH as usize]> = Vec::new();

    vec_rows.resize(HEIGHT as usize, [bg_color; WIDTH as usize]);

    let mut _p = Point::new(0, 0);
    let mut side_toggle: bool;

    // data[0][10] = fg_color;
    // data[5][10] = fg_color;
    // data[0][20] = fg_color;
    // println!("loop start");

    while !input.get_quit() {
        time = Instant::now();
        input.update();

        //NOTE: INPUT
        let _time_input = Instant::now().duration_since(time).as_micros();
        time = Instant::now();

        let tmp = f64::sin(framecount as f64 / COLORSPEED) * 127.0 + 127.0;
        neo_color.r = tmp as u8;
        let tmp = f64::sin(framecount as f64 / COLORSPEED + (PI * 0.67)) * 127.0 + 127.0;
        neo_color.g = tmp as u8;
        let tmp = f64::sin(framecount as f64 / COLORSPEED + (PI * 1.34)) * 127.0 + 127.0;
        neo_color.b = tmp as u8;

        if framecount % 2 == 0 {
            side_toggle = false;
        } else {
            side_toggle = true
        }
        vec_old_rows = vec_rows.clone();

        for (y, row) in vec_old_rows.iter().enumerate() {
            if y == HEIGHT as usize - 1 {
                break;
            }
            for (x, cell) in row.iter().enumerate() {
                if *cell != bg_color {
                    if vec_old_rows[y + 1][x] == bg_color {
                        vec_rows[y + 1][x] = *cell;
                        vec_rows[y][x] = bg_color;
                    } else if side_toggle {
                        if (x < WIDTH as usize - 1) && (vec_old_rows[y + 1][x + 1] == bg_color) {
                            vec_rows[y + 1][x + 1] = *cell;
                            vec_rows[y][x] = bg_color;
                        } else if (x > 0) && (vec_old_rows[y + 1][x - 1] == bg_color) {
                            vec_rows[y + 1][x - 1] = *cell;
                            vec_rows[y][x] = bg_color;
                        }
                    } else {
                        if (x > 0) && (vec_old_rows[y + 1][x - 1] == bg_color) {
                            vec_rows[y + 1][x - 1] = *cell;
                            vec_rows[y][x] = bg_color;
                        } else if (x < WIDTH as usize - 1)
                            && (vec_old_rows[y + 1][x + 1] == bg_color)
                        {
                            vec_rows[y + 1][x + 1] = *cell;
                            vec_rows[y][x] = bg_color;
                        }
                    }
                }
            }
        }

        //NOTE: data done
        let _time_data = Instant::now().duration_since(time).as_micros();
        time = Instant::now();

        if input.mouse_left
            && (input.mouse_y >= 0 && input.mouse_y < HEIGHT as i32)
            && (input.mouse_x >= 0 && input.mouse_x < WIDTH as i32)
        {
            vec_rows[input.mouse_y as usize % HEIGHT as usize]
                [input.mouse_x as usize % WIDTH as usize] = neo_color;
        }

        canvas.set_draw_color(bg_color);
        canvas.clear();
        canvas.set_draw_color(fg_color);
        for (y, row) in vec_rows.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell == bg_color {
                    continue;
                }
                canvas.set_draw_color(*cell);
                // p.x = x as i32;
                // p.y = y as i32;
                let _ = canvas.draw_point(Point::new(x as i32, y as i32));
            }
        }

        //NOTE: Version 2
        //
        // let mut points: Vec<Point> = Vec::new();
        // for y in 0..HEIGHT {
        //     for x in 0..WIDTH {
        //         if data[y as usize][x as usize] == bg_color {
        //             continue;
        //         }
        //         // canvas.set_draw_color(data[y as usize][x as usize]);
        //         points.push(Point::new(x as i32, y as i32));
        //         // let _ =canvas.draw_point(Point::new(x as i32,y as i32));
        //     }
        // }
        // canvas.set_draw_color(neo_color);
        // let _ = canvas.draw_points(points.as_slice());

        canvas.present();
        framecount += 1;

        //NOTE: drawing done
        let _time_draw = Instant::now().duration_since(time).as_micros();

        // if framecount % 50 == 0 {
        //     println!("input {} data {} draw {}", time_input, time_data, time_draw);
        // }

        // thread::sleep(Duration::from_millis(10))
    }
}
