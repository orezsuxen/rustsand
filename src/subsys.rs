#[allow(unused)]
use sdl2::{
    pixels::{Color, PixelFormatEnum},
    render::{Canvas, Texture, TextureCreator},
    surface::Surface,
    video::{Window, WindowContext},
    EventPump, Sdl,
};

pub fn create_subsys(
    win_width: u32,
    win_height: u32,
    logi_width: u32,
    logi_height: u32,
) -> (Sdl, Canvas<Window>, TextureCreator<WindowContext>) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsys = sdl_context.video().unwrap();

    let window = video_subsys
        .window("", win_width, win_height)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        // .present_vsync()
        // .target_texture()
        .build()
        .unwrap();

    canvas.set_logical_size(logi_width, logi_height).unwrap();
    canvas.set_integer_scale(true).unwrap();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();

    let texture_creator = canvas.texture_creator();

    (sdl_context, canvas, texture_creator)
}
