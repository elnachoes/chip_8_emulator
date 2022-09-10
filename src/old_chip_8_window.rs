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

static OFF_COLOR : Color = Color::RGB(255,255,255);
static ON_COLOR : Color = Color::RGB(0,0,0);

pub struct OldChip8Window {
    pub buffer : Vec<Vec<bool>>,
    pub sdl_context : Sdl,
    pub video_subsystem : VideoSubsystem,
    pub canvas : WindowCanvas,
    pub event_pump : EventPump,
}

impl OldChip8Window {
    const SCREEN_WIDTH : usize = 64;
    const SCREEN_HEIGHT : usize = 32;
    const PIXEL_SIZE : usize = 20;

    pub fn new() -> OldChip8Window {
        let buffer = vec![vec![false; Self::SCREEN_WIDTH]; Self::SCREEN_HEIGHT];

        let sdl_context = sdl2::init().unwrap();
        
        let video_subsystem = sdl_context.video().unwrap();
    
        let window = video_subsystem.window(
            "chip-8-emulator", 
            (Self::SCREEN_WIDTH * Self::PIXEL_SIZE) as u32,
            (Self::SCREEN_HEIGHT * Self::PIXEL_SIZE) as u32,)
            .position_centered().build().unwrap();

        let canvas = window.into_canvas().build().unwrap();
    
        let event_pump = sdl_context.event_pump().unwrap();

        OldChip8Window{
            buffer : buffer,
            sdl_context : sdl_context,
            video_subsystem : video_subsystem,
            canvas : canvas,
            event_pump : event_pump,
        }
    }

    //todo : call this as a thread that updates every 60th of a second
    //this may not entirely be nessesary 
    pub fn run_window(&mut self) {
        loop {
            self.handle_input();
            self.draw_canvas();
        }
    }

    fn handle_input(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    std::process::exit(0);
                },
                _ => {}
            }
        }
    }

    fn draw_canvas(&mut self) {
        self.canvas.set_draw_color(OFF_COLOR);
        self.canvas.clear();
        
        self.canvas.set_draw_color(ON_COLOR);
        
        let mut horizontal_position : usize = 0;
        for i in self.buffer.iter() {
            let mut vertical_position : usize = 0;
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
                vertical_position += 1;
            }
            horizontal_position += 1;
        }

        self.canvas.present();
    }
}