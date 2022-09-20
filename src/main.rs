use chip_8_emulator::{Chip8};

fn main() {
    let mut x = Chip8::new();
    // x.pc_reg = 0;
    // x.load_rom_from_bin(&String::from("C:\\Sudo Desktop\\programming\\RustStuffs\\chip_8_emulator\\fullgames\\1dcell.ch8"));
    x.load_rom_from_bin(&String::from("C:\\Sudo Desktop\\programming\\RustStuffs\\chip_8_emulator\\fullgames\\pumpkindressup.ch8"));
    x.load_font();
    x.start_processor_loop();


    // let mut window = chip_8_emulator::Chip8Window::new();

    // loop {
    //     if let chip_8_emulator::Keyboard::Key0 = window.handle_input() {
    //         println!("farts");
    //     }
    //     if let chip_8_emulator::Keyboard::Key3 = window.handle_input() {
    //         println!("ass");
    //     }
    // }
}