use sdl2::{
    render::WindowCanvas,
    Sdl, 
    VideoSubsystem,
    pixels::Color,
    rect::Rect,
    EventPump,
    event::Event,
    keyboard::Keycode, AudioSubsystem, audio::{AudioCallback, AudioSpecDesired, AudioDevice}
};

use crate::Keyboard;

static OFF_COLOR : Color = Color::RGB(255,255,255);
static ON_COLOR : Color = Color::RGB(0,0,0);

pub struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}


pub struct Chip8Window {
    pub sdl_context : Sdl,
    pub video_subsystem : VideoSubsystem,
    pub canvas : WindowCanvas,
    pub event_pump : EventPump,
    pub off_color : Color,
    pub on_color : Color,

    pub audio_subsystem : AudioSubsystem,
    pub audio_spec : AudioSpecDesired,
    pub audio_device : AudioDevice<SquareWave>
}

impl Chip8Window {
    const SCREEN_WIDTH : usize = 64;
    const SCREEN_HEIGHT : usize = 32;
    const PIXEL_SIZE : usize = 20;

    pub fn new() -> Chip8Window {
        let sdl_context = sdl2::init().unwrap();
        
        let video_subsystem = sdl_context.video().unwrap();
        let audio_subsystem = sdl_context.audio().unwrap();

        let audio_spec = AudioSpecDesired {
            freq: Some(44110),
            channels: Some(1),  // mono audio
            samples: None
        };

        let audio_device = audio_subsystem.open_playback(None, &audio_spec, |spec| {
            // audio callback
            SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25
            }
        }).unwrap();

        let window = video_subsystem.window(
            "rip8", 
            (Self::SCREEN_WIDTH * Self::PIXEL_SIZE) as u32,
            (Self::SCREEN_HEIGHT * Self::PIXEL_SIZE) as u32,)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
    
        let event_pump = sdl_context.event_pump().unwrap();

        Chip8Window{
            sdl_context : sdl_context,
            video_subsystem : video_subsystem,
            canvas : canvas,
            event_pump : event_pump,
            off_color : OFF_COLOR,
            on_color : ON_COLOR,

            audio_subsystem : audio_subsystem,
            audio_spec : audio_spec,
            audio_device : audio_device,
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
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    std::process::exit(0);
                },
                Event::KeyDown { keycode: Some(Keycode::Num1), .. } => {
                    return Keyboard::Key1
                },
                Event::KeyDown { keycode: Some(Keycode::Num2), .. } => {
                    return Keyboard::Key2
                },
                Event::KeyDown { keycode: Some(Keycode::Num3), .. } => {
                    return Keyboard::Key3
                },
                Event::KeyDown { keycode: Some(Keycode::Num4), .. } => {
                    return Keyboard::KeyC
                },
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                    return Keyboard::Key4
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    return Keyboard::Key5
                },
                Event::KeyDown { keycode: Some(Keycode::E), .. } => {
                    return Keyboard::Key6
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    return Keyboard::KeyD
                },
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    return Keyboard::Key7
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    return Keyboard::Key8
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    return Keyboard::Key9
                },
                Event::KeyDown { keycode: Some(Keycode::F), .. } => {
                    return Keyboard::KeyE
                },
                Event::KeyDown { keycode: Some(Keycode::Z), .. } => {
                    return Keyboard::KeyA
                },
                Event::KeyDown { keycode: Some(Keycode::X), .. } => {
                    return Keyboard::Key0
                },
                Event::KeyDown { keycode: Some(Keycode::C), .. } => {
                    return Keyboard::KeyB
                },
                Event::KeyDown { keycode: Some(Keycode::V), .. } => {
                    return Keyboard::KeyF
                },
                _ => {}
            }
        }

        Keyboard::None
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