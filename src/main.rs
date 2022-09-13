use chip_8_emulator::{Chip8};

fn main() {
    let mut x = Chip8::new();
    x.load_rom_from_radix_at_512(&String::from("C:\\Sudo Desktop\\programming\\RustStuffs\\chip_8_emulator\\testroms\\test_font.chip8"));
    
    x.load_font();
    // println!("")
    x.start_processor_loop();
}