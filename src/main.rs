use chip_8_emulator::{ NewChip8, Chip8Window };
use std::{time, thread};

fn run_emulator(cycles_per_frame : u32, rom_path : &String) {
    const FRAME_TIME : f64 = 1_f64 / 60_f64;
    
    let mut chip8 = NewChip8::new();
    chip8.load_rom_from_bin(rom_path);
    chip8.load_font();

    let mut chip8_window = Chip8Window::new();

    chip8_window.invert_colors();

    loop {

        let start_frame_time = time::Instant::now();

        let keyboard = chip8_window.handle_input();

        for _ in 0..cycles_per_frame {
            if let Err(_) = chip8.processor_frame(keyboard.clone()) {
                return
            }
        }

        chip8_window.draw_canvas(chip8.display_buffer.clone());
        
        chip8.update_timers();

        let remaining_frame_time = FRAME_TIME - start_frame_time.elapsed().as_secs_f64();

        thread::sleep(time::Duration::from_secs_f64(remaining_frame_time))
    }
}


fn main() {
    // run_emulator(30, &String::from("C:\\Sudo Desktop\\programming\\RustStuffs\\chip_8_emulator\\testroms\\test_opcode.ch8"));
    run_emulator(1000, &String::from("C:\\Sudo Desktop\\programming\\RustStuffs\\chip_8_emulator\\fullgames\\chipquarium.ch8"));
}