#![allow(unused)]
use sdl2::{event::Event, keyboard::Scancode, mouse::MouseButton, EventPump};

pub struct Input {
    eventpump: EventPump,
    global_quit: bool,
    keys: [bool; 512],
    pub mouse_x: i32,
    pub mouse_y: i32,
    pub mouse_left: bool,
}

impl Input {
    pub fn build(eventpump: EventPump) -> Input {
        Input {
            global_quit: false,
            eventpump,
            keys: [false; 512],
            mouse_x: 0,
            mouse_y: 0,
            mouse_left: false,
        }
    }

    pub fn get_quit(&self) -> &bool {
        &self.global_quit
    }

    pub fn update(&mut self) {
        for event in self.eventpump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    scancode: Some(Scancode::Escape),
                    ..
                } => {
                    self.global_quit = true;
                }

                Event::KeyDown {
                    scancode: Some(scancode),
                    repeat: false,
                    ..
                } => {
                    self.keys[scancode as usize] = true;
                }
                Event::KeyUp {
                    scancode: Some(scancode),
                    repeat: false,
                    ..
                } => {
                    self.keys[scancode as usize] = false;
                }
                Event::MouseMotion { x, y, .. } => {
                    self.mouse_x = x;
                    self.mouse_y = y;
                    // println!("x,y is {}, {}", x,y);
                }
                Event::MouseButtonDown { mouse_btn, .. } => {
                    if mouse_btn == MouseButton::Left {
                        self.mouse_left = true;
                    }
                    // println!("mouse button is {:?} clicks is {}",mouse_btn, clicks);
                    // println!("x,y is {},{}", x,y);
                }
                Event::MouseButtonUp { mouse_btn, .. } => {
                    if mouse_btn == MouseButton::Left {
                        self.mouse_left = false;
                    }
                }
                _ => {}
            }
        }
    }

    pub fn get_key_state(&mut self, scancode: Scancode) -> bool {
        self.keys[scancode as usize]
    }
}
