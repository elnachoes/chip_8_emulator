use sdl2::keyboard::Scancode;

use sdl2::{
    render::WindowCanvas,
    Sdl, 
    VideoSubsystem,
    pixels::Color,
    rect::Rect,
    EventPump,
    event::Event,
    keyboard::Keycode
};

use std::time;

use crate::Keyboard;

static OFF_COLOR : Color = Color::RGB(255,255,255);
static ON_COLOR : Color = Color::RGB(0,0,0);

pub struct Chip8Window {
    pub sdl_context : Sdl,
    pub video_subsystem : VideoSubsystem,
    pub canvas : WindowCanvas,
    pub event_pump : EventPump,
    pub off_color : Color,
    pub on_color : Color,
}

impl Chip8Window {
    const SCREEN_WIDTH : usize = 64;
    const SCREEN_HEIGHT : usize = 32;
    const PIXEL_SIZE : usize = 20;

    pub fn new() -> Chip8Window {
        let sdl_context = sdl2::init().unwrap();
        
        let video_subsystem = sdl_context.video().unwrap();
    
        let window = video_subsystem.window(
            "chip-8-emulator", 
            (Self::SCREEN_WIDTH * Self::PIXEL_SIZE) as u32,
            (Self::SCREEN_HEIGHT * Self::PIXEL_SIZE) as u32,)
            .position_centered().build().unwrap();

        let canvas = window.into_canvas().build().unwrap();
    
        let event_pump = sdl_context.event_pump().unwrap();

        Chip8Window{
            sdl_context : sdl_context,
            video_subsystem : video_subsystem,
            canvas : canvas,
            event_pump : event_pump,
            off_color : OFF_COLOR,
            on_color : ON_COLOR,
        }
    }

    pub fn invert_colors(&mut self) {
        self.off_color = if self.off_color == OFF_COLOR {
            ON_COLOR
        } else {
            OFF_COLOR
        };

        self.on_color = if self.on_color == OFF_COLOR {
            ON_COLOR
        } else {
            OFF_COLOR
        }
    }

    // have this return a set of pressed keys back to the chip8
    pub fn handle_input(&mut self) -> Keyboard {
        let mut keyboard_state = Keyboard::None;
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    std::process::exit(0);
                },

                Event::KeyDown { keycode : Some(Keycode::Kp0), .. } => {
                    keyboard_state = Keyboard::Key0
                }
                
                Event::KeyDown { keycode : Some(Keycode::Kp1), .. } => {
                    keyboard_state = Keyboard::Key1
                }
                
                Event::KeyDown { keycode : Some(Keycode::Kp2), .. } => {
                    keyboard_state = Keyboard::Key2
                }
                
                Event::KeyDown { keycode : Some(Keycode::Kp3), .. } => {
                    keyboard_state = Keyboard::Key3
                }
                
                Event::KeyDown { keycode : Some(Keycode::Kp4), .. } => {
                    keyboard_state = Keyboard::Key4
                }
                
                Event::KeyDown { keycode : Some(Keycode::Kp5), .. } => {
                    keyboard_state = Keyboard::Key5
                }
                
                Event::KeyDown { keycode : Some(Keycode::Kp6), .. } => {
                    keyboard_state = Keyboard::Key6
                }
                
                Event::KeyDown { keycode : Some(Keycode::Kp7), .. } => {
                    keyboard_state = Keyboard::Key7
                }
                
                Event::KeyDown { keycode : Some(Keycode::Kp8), .. } => {
                    keyboard_state = Keyboard::Key8
                }
                
                Event::KeyDown { keycode : Some(Keycode::Kp9), .. } => {
                    keyboard_state = Keyboard::Key9
                }
                
                Event::KeyDown { keycode : Some(Keycode::KpPeriod), .. } => {
                    keyboard_state = Keyboard::KeyA
                }
                
                Event::KeyDown { keycode : Some(Keycode::KpEnter), .. } => {
                    keyboard_state = Keyboard::KeyB
                }

                Event::KeyDown { keycode : Some(Keycode::KpPlus), .. } => {
                    keyboard_state = Keyboard::KeyC
                }
                
                Event::KeyDown { keycode : Some(Keycode::KpMinus), .. } => {
                    keyboard_state = Keyboard::KeyD
                    
                }
                
                Event::KeyDown { keycode : Some(Keycode::KpMultiply), .. } => {
                    keyboard_state = Keyboard::KeyE
                    
                }
                
                Event::KeyDown { keycode : Some(Keycode::KpDivide), .. } => {
                    keyboard_state = Keyboard::KeyF
                }

                _ => {}
            }
        }
        keyboard_state
    }

    pub fn draw_canvas(&mut self, buffer : Vec<Vec<bool>>) {
        self.canvas.set_draw_color(self.off_color);
        self.canvas.clear();
        
        self.canvas.set_draw_color(self.on_color);
        
        let mut vertical_position : usize = 0;
        for i in buffer.iter() {
            let mut horizontal_position : usize = 0;
            for j in i.iter() {
                if *j {
                    let rect = Rect::new(
                        (horizontal_position * Self::PIXEL_SIZE) as i32,
                        (vertical_position * Self::PIXEL_SIZE) as i32,
                        Self::PIXEL_SIZE as u32,
                        Self::PIXEL_SIZE as u32,
                    );
                    match self.canvas.fill_rect(rect) {
                        Err(string) => panic!("drawing error happened : '{}'", string),
                        _ => {}
                    }
                }
                horizontal_position += 1;
            }
            vertical_position += 1;
        }

        self.canvas.present();
    }
}